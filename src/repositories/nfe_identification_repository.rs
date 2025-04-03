use crate::models::nfe_identification::{CreateNFeIdentification, NFeIdentification};
use crate::services::cache_service::CacheService;
use chrono::{DateTime, NaiveDateTime, Utc};
use oracle::{sql_type::ToSql, Connection};
use redis::aio::ConnectionManager;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, instrument};
use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub enum RepositoryError {
    OracleError(oracle::Error),
    NotFound,
    CreationFailed,
    UpdateFailed,
    InvalidUuid(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::OracleError(e) => write!(f, "Oracle error: {}", e),
            RepositoryError::NotFound => write!(f, "Record not found"),
            RepositoryError::CreationFailed => write!(f, "Failed to create record"),
            RepositoryError::UpdateFailed => write!(f, "Failed to update record"),
            RepositoryError::InvalidUuid(msg) => write!(f, "Invalid UUID: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<oracle::Error> for RepositoryError {
    fn from(err: oracle::Error) -> Self {
        RepositoryError::OracleError(err)
    }
}

pub struct NFeIdentificationRepository {
    conn: Connection,
    cache: Arc<CacheService>,
}

impl NFeIdentificationRepository {
    pub fn new(conn: Connection, redis_manager: ConnectionManager) -> Self {
        Self {
            conn,
            cache: Arc::new(CacheService::new(redis_manager)),
        }
    }

    #[instrument(skip(self))]
    pub async fn find_all(
        &self,
        page: u32,
        page_size: u32,
        nat_op: Option<String>,
        n_nf: Option<String>,
        tp_nf: Option<String>,
        dh_emi: Option<String>,
        search: Option<String>,
    ) -> Result<(Vec<NFeIdentification>, u64), RepositoryError> {
        info!(
            "Fetching NFe identifications with pagination - page: {}, page_size: {}",
            page, page_size
        );

        // Try to get from cache first
        let cache_key = format!(
            "nfe:list:{}:{}:{}:{}:{}:{}:{}",
            page,
            page_size,
            nat_op.as_deref().unwrap_or(""),
            n_nf.as_deref().unwrap_or(""),
            tp_nf.as_deref().unwrap_or(""),
            dh_emi.as_deref().unwrap_or(""),
            search.as_deref().unwrap_or("")
        );
        if let Ok(Some(cached)) = self
            .cache
            .get::<(Vec<NFeIdentification>, u64)>(&cache_key)
            .await
        {
            info!("Cache hit for NFe identifications list");
            return Ok(cached);
        }

        // Calculate offset
        let offset = (page - 1) * page_size;

        // Build WHERE clause for filters
        let mut where_clauses = Vec::new();
        let mut params: Vec<(String, String)> = Vec::new();

        if let Some(nat_op) = &nat_op {
            where_clauses.push("NATOP LIKE :nat_op");
            params.push(("nat_op".to_string(), format!("%{}%", nat_op)));
        }

        if let Some(n_nf) = &n_nf {
            where_clauses.push("NNF LIKE :n_nf");
            params.push(("n_nf".to_string(), format!("%{}%", n_nf)));
        }

        if let Some(tp_nf) = &tp_nf {
            where_clauses.push("TPNF = :tp_nf");
            params.push(("tp_nf".to_string(), tp_nf.to_string()));
        }

        if let Some(dh_emi) = &dh_emi {
            where_clauses.push("TO_CHAR(DHEMI, 'YYYY-MM-DD') = :dh_emi");
            params.push(("dh_emi".to_string(), dh_emi.to_string()));
        }

        if let Some(search) = &search {
            where_clauses.push("(NATOP LIKE :search OR NNF LIKE :search OR TPNF LIKE :search)");
            params.push(("search".to_string(), format!("%{}%", search)));
        }

        let where_clause = if !where_clauses.is_empty() {
            format!("WHERE {}", where_clauses.join(" AND "))
        } else {
            String::new()
        };

        // First, get the total count
        let count_sql = format!(
            "SELECT COUNT(*) as count FROM nfe_identifications {}",
            where_clause
        );
        let mut count_stmt = self.conn.statement(&count_sql).build()?;
        for (name, value) in &params {
            count_stmt.bind(name.as_str(), value)?;
        }
        let mut rows = count_stmt.query(&[])?;
        let total_count: u64 = if let Some(row) = rows.next() {
            let row = row?;
            row.get("count")?
        } else {
            0
        };

        // Then get the paginated results
        let sql = format!(
            r#"
            SELECT * FROM (
                SELECT a.*, ROWNUM rnum FROM (
                    SELECT 
                        RAWTOHEX(INTERNALKEY) as internal_key,
                        CUF as c_uf,
                        CNF as c_nf,
                        NATOP as nat_op,
                        MOD_ as mod_,
                        SERIE as serie,
                        NNF as n_nf,
                        TO_CHAR(DHEMI, 'YYYY-MM-DD HH24:MI:SS.FF3') as dh_emi,
                        TO_CHAR(DHSAIENT, 'YYYY-MM-DD HH24:MI:SS.FF3') as dh_sai_ent,
                        TPNF as tp_nf,
                        IDDEST as id_dest,
                        CMUNFG as c_mun_fg,
                        TPIMP as tp_imp,
                        TPEMIS as tp_emis,
                        CDV as c_dv,
                        TPAMB as tp_amb,
                        FINNFE as fin_nfe,
                        INDFINAL as ind_final,
                        INDPRES as ind_pres,
                        PROCEMI as proc_emi,
                        VERPROC as ver_proc,
                        TO_CHAR(DHCONT, 'YYYY-MM-DD HH24:MI:SS.FF3') as dh_cont,
                        TO_CHAR(CREATEDAT, 'YYYY-MM-DD HH24:MI:SS.FF3') as created_at,
                        TO_CHAR(UPDATEDAT, 'YYYY-MM-DD HH24:MI:SS.FF3') as updated_at
                    FROM nfe_identifications
                    {}
                    ORDER BY DHEMI DESC
                ) a WHERE ROWNUM <= :offset + :page_size
            ) WHERE rnum > :offset
            "#,
            where_clause
        );

        let mut stmt = self.conn.statement(&sql).build()?;
        for (name, value) in &params {
            stmt.bind(name.as_str(), value)?;
        }
        stmt.bind("offset", &(offset + page_size))?;
        stmt.bind("page_size", &page_size)?;
        stmt.bind("offset", &offset)?;
        let rows = stmt.query(&[])?;

        let mut identifications = Vec::new();
        for row_result in rows {
            let row = row_result?;
            let internal_key_hex: String = row.get("internal_key")?;
            let dh_emi_str: String = row.get("dh_emi")?;
            let dh_sai_ent_str: Option<String> = row.get("dh_sai_ent")?;
            let dh_cont_str: Option<String> = row.get("dh_cont")?;
            let created_at_str: String = row.get("created_at")?;
            let updated_at_str: String = row.get("updated_at")?;

            let internal_key = Uuid::parse_str(&internal_key_hex)
                .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

            let dh_emi = NaiveDateTime::parse_from_str(&dh_emi_str, "%Y-%m-%d %H:%M:%S%.3f")
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|_| Utc::now());

            let dh_sai_ent = dh_sai_ent_str.map(|s| {
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now())
            });

            let dh_cont = dh_cont_str.map(|s| {
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now())
            });

            let created_at =
                NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now());

            let updated_at =
                NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now());

            let identification = NFeIdentification {
                internal_key: internal_key.to_string(),
                c_uf: row.get("c_uf")?,
                c_nf: row.get("c_nf")?,
                nat_op: row.get("nat_op")?,
                mod_: row.get("mod_")?,
                serie: row.get("serie")?,
                n_nf: row.get("n_nf")?,
                dh_emi,
                dh_sai_ent,
                dh_cont,
                tp_nf: row.get("tp_nf")?,
                id_dest: row.get("id_dest")?,
                c_mun_fg: row.get("c_mun_fg")?,
                tp_imp: row.get("tp_imp")?,
                tp_emis: row.get("tp_emis")?,
                c_dv: row.get("c_dv")?,
                tp_amb: row.get("tp_amb")?,
                fin_nfe: row.get("fin_nfe")?,
                ind_final: row.get("ind_final")?,
                ind_pres: row.get("ind_pres")?,
                proc_emi: row.get("proc_emi")?,
                ver_proc: row.get("ver_proc")?,
                x_justificativa: None,
                created_at,
                updated_at,
            };
            identifications.push(identification);
        }

        // Cache the results
        if let Err(e) = self
            .cache
            .set(
                &cache_key,
                &(identifications.clone(), total_count),
                Some(Duration::from_secs(300)),
            )
            .await
        {
            error!("Failed to cache NFe identifications: {}", e);
        }

        Ok((identifications, total_count))
    }

    #[instrument(skip(self), fields(internal_key = %internal_key))]
    pub async fn find_by_id(
        &self,
        internal_key: &str,
    ) -> Result<Option<NFeIdentification>, RepositoryError> {
        info!("Fetching NFe identification by ID");

        // Try to get from cache first
        let cache_key = format!("nfe:{}", internal_key);
        if let Ok(Some(cached)) = self.cache.get::<NFeIdentification>(&cache_key).await {
            info!("Cache hit for NFe identification {}", internal_key);
            return Ok(Some(cached));
        }

        let uuid = Uuid::parse_str(internal_key)
            .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

        // Format UUID for Oracle HEXTORAW (remove hyphens)
        let oracle_uuid = uuid.to_string().replace('-', "");

        let sql = r#"
            SELECT 
                RAWTOHEX(INTERNALKEY),
                CUF,
                CNF,
                NATOP,
                MOD_,
                SERIE,
                NNF,
                TO_CHAR(DHEMI, 'YYYY-MM-DD HH24:MI:SS.FF3'),
                TO_CHAR(DHSAIENT, 'YYYY-MM-DD HH24:MI:SS.FF3'),
                TO_CHAR(DHCONT, 'YYYY-MM-DD HH24:MI:SS.FF3'),
                TPNF,
                IDDEST,
                CMUNFG,
                TPIMP,
                TPEMIS,
                CDV,
                TPAMB,
                FINNFE,
                INDFINAL,
                INDPRES,
                PROCEMI,
                VERPROC,
                TO_CHAR(CREATEDAT, 'YYYY-MM-DD HH24:MI:SS.FF3'),
                TO_CHAR(UPDATEDAT, 'YYYY-MM-DD HH24:MI:SS.FF3')
            FROM nfe_identifications
            WHERE INTERNALKEY = HEXTORAW(:1)
        "#;

        let mut stmt = self.conn.statement(sql).build()?;
        let mut rows = stmt.query(&[&oracle_uuid])?;

        if let Some(row_result) = rows.next() {
            let row = row_result?;
            let internal_key_hex: String = row.get(0)?;
            let dh_emi_str: String = row.get(7)?;
            let dh_sai_ent_str: Option<String> = row.get(8)?;
            let dh_cont_str: Option<String> = row.get(9)?;
            let created_at_str: String = row.get(20)?;
            let updated_at_str: String = row.get(21)?;

            let internal_key = Uuid::parse_str(&internal_key_hex)
                .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

            let dh_emi = NaiveDateTime::parse_from_str(&dh_emi_str, "%Y-%m-%d %H:%M:%S%.3f")
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|_| Utc::now());

            let dh_sai_ent = dh_sai_ent_str.map(|s| {
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now())
            });

            let dh_cont = dh_cont_str.map(|s| {
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now())
            });

            let created_at =
                NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now());

            let updated_at =
                NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now());

            let identification = NFeIdentification {
                internal_key: internal_key.to_string(),
                c_uf: row.get(1)?,
                c_nf: row.get(2)?,
                nat_op: row.get(3)?,
                mod_: row.get(4)?,
                serie: row.get(5)?,
                n_nf: row.get(6)?,
                dh_emi,
                dh_sai_ent,
                dh_cont,
                tp_nf: row.get(10)?,
                id_dest: row.get(11)?,
                c_mun_fg: row.get(12)?,
                tp_imp: row.get(13)?,
                tp_emis: row.get(14)?,
                c_dv: row.get(15)?,
                tp_amb: row.get(16)?,
                fin_nfe: row.get(17)?,
                ind_final: row.get(18)?,
                ind_pres: row.get(19)?,
                proc_emi: row.get(20)?,
                ver_proc: row.get(21)?,
                x_justificativa: None,
                created_at,
                updated_at,
            };

            // Cache the result
            if let Err(e) = self
                .cache
                .set(&cache_key, &identification, Some(Duration::from_secs(300)))
                .await
            {
                error!("Failed to cache NFe identification: {}", e);
            }

            info!("Found NFe identification with ID {}", internal_key);
            Ok(Some(identification))
        } else {
            info!("No NFe identification found with ID {}", internal_key);
            Ok(None)
        }
    }

    #[instrument(skip(self, identification))]
    pub async fn create(
        &self,
        identification: &CreateNFeIdentification,
    ) -> Result<NFeIdentification, RepositoryError> {
        info!("Creating new NFe identification");
        debug!("Input data: {:?}", identification);

        let sql = r#"
            INSERT INTO nfe_identifications (
                INTERNALKEY,
                CUF,
                CNF,
                NATOP,
                MOD_,
                SERIE,
                NNF,
                DHEMI,
                DHSAIENT,
                DHCONT,
                TPNF,
                IDDEST,
                CMUNFG,
                TPIMP,
                TPEMIS,
                CDV,
                TPAMB,
                FINNFE,
                INDFINAL,
                INDPRES,
                PROCEMI,
                VERPROC
            ) VALUES (
                HEXTORAW(:1),
                :2,
                :3,
                :4,
                :5,
                :6,
                :7,
                TO_TIMESTAMP(:8, 'YYYY-MM-DD HH24:MI:SS.FF3'),
                TO_TIMESTAMP(:9, 'YYYY-MM-DD HH24:MI:SS.FF3'),
                TO_TIMESTAMP(:10, 'YYYY-MM-DD HH24:MI:SS.FF3'),
                :11,
                :12,
                :13,
                :14,
                :15,
                :16,
                :17,
                :18,
                :19,
                :20,
                :21,
                :22
            )
        "#;

        let mut stmt = self.conn.statement(sql).build()?;
        let internal_key = Uuid::new_v4();

        // Format UUID for Oracle HEXTORAW (remove hyphens)
        let oracle_uuid = internal_key.to_string().replace('-', "");

        let dh_emi_str = identification
            .dh_emi
            .format("%Y-%m-%d %H:%M:%S%.3f")
            .to_string();
        let dh_sai_ent_str = identification
            .dh_sai_ent
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        let dh_cont_str = identification
            .dh_cont
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string());

        debug!(
            "Executing SQL with parameters: internal_key={}, dh_emi={}, dh_sai_ent={:?}, dh_cont={:?}",
            oracle_uuid, dh_emi_str, dh_sai_ent_str, dh_cont_str
        );

        let result = stmt.execute(&[
            &oracle_uuid,
            &identification.c_uf,
            &identification.c_nf,
            &identification.nat_op,
            &identification.mod_,
            &identification.serie,
            &identification.n_nf,
            &dh_emi_str,
            &dh_sai_ent_str,
            &dh_cont_str,
            &identification.tp_nf,
            &identification.id_dest,
            &identification.c_mun_fg,
            &identification.tp_imp,
            &identification.tp_emis,
            &identification.c_dv,
            &identification.tp_amb,
            &identification.fin_nfe,
            &identification.ind_final,
            &identification.ind_pres,
            &identification.proc_emi,
            &identification.ver_proc,
        ]);

        match result {
            Ok(_) => {
                info!(
                    "Successfully created NFe identification with ID {}",
                    internal_key
                );
                let created = self
                    .find_by_id(&internal_key.to_string())
                    .await?
                    .ok_or(RepositoryError::CreationFailed)?;

                // Invalidate list cache
                if let Err(e) = self.cache.delete("nfe:list:*").await {
                    error!("Failed to invalidate list cache: {}", e);
                }

                Ok(created)
            }
            Err(e) => {
                error!("Failed to create NFe identification: {}", e);
                Err(RepositoryError::OracleError(e))
            }
        }
    }

    #[instrument(skip(self, identification), fields(internal_key = %internal_key))]
    pub async fn update(
        &self,
        internal_key: &str,
        identification: &NFeIdentification,
    ) -> Result<NFeIdentification, RepositoryError> {
        info!("Updating NFe identification with ID {}", internal_key);
        debug!("Update data: {:?}", identification);

        // Parse the UUID to ensure it's valid
        let uuid = Uuid::parse_str(internal_key)
            .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

        // Format UUID for Oracle HEXTORAW (remove hyphens)
        let oracle_uuid = uuid.to_string().replace('-', "");

        let sql = r#"
            UPDATE nfe_identifications
            SET
                CUF = NVL(:1, CUF),
                CNF = NVL(:2, CNF),
                NATOP = NVL(:3, NATOP),
                MOD_ = NVL(:4, MOD_),
                SERIE = NVL(:5, SERIE),
                NNF = NVL(:6, NNF),
                DHEMI = NVL(TO_TIMESTAMP(:7, 'YYYY-MM-DD HH24:MI:SS.FF3'), DHEMI),
                DHSAIENT = NVL(TO_TIMESTAMP(:8, 'YYYY-MM-DD HH24:MI:SS.FF3'), DHSAIENT),
                DHCONT = NVL(TO_TIMESTAMP(:9, 'YYYY-MM-DD HH24:MI:SS.FF3'), DHCONT),
                TPNF = NVL(:10, TPNF),
                IDDEST = NVL(:11, IDDEST),
                CMUNFG = NVL(:12, CMUNFG),
                TPIMP = NVL(:13, TPIMP),
                TPEMIS = NVL(:14, TPEMIS),
                CDV = NVL(:15, CDV),
                TPAMB = NVL(:16, TPAMB),
                FINNFE = NVL(:17, FINNFE),
                INDFINAL = NVL(:18, INDFINAL),
                INDPRES = NVL(:19, INDPRES),
                PROCEMI = NVL(:20, PROCEMI),
                VERPROC = NVL(:21, VERPROC)
            WHERE INTERNALKEY = HEXTORAW(:22)
        "#;

        let mut stmt = self.conn.statement(sql).build()?;

        let dh_emi_str = identification
            .dh_emi
            .format("%Y-%m-%d %H:%M:%S%.3f")
            .to_string();
        let dh_sai_ent_str = identification
            .dh_sai_ent
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        let dh_cont_str = identification
            .dh_cont
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string());

        debug!(
            "Executing SQL with parameters: dh_emi={}, dh_sai_ent={:?}, dh_cont={:?}, uuid={}",
            dh_emi_str, dh_sai_ent_str, dh_cont_str, oracle_uuid
        );

        let result = stmt.execute(&[
            &identification.c_uf,
            &identification.c_nf,
            &identification.nat_op,
            &identification.mod_,
            &identification.serie,
            &identification.n_nf,
            &dh_emi_str,
            &dh_sai_ent_str,
            &dh_cont_str,
            &identification.tp_nf,
            &identification.id_dest,
            &identification.c_mun_fg,
            &identification.tp_imp,
            &identification.tp_emis,
            &identification.c_dv,
            &identification.tp_amb,
            &identification.fin_nfe,
            &identification.ind_final,
            &identification.ind_pres,
            &identification.proc_emi,
            &identification.ver_proc,
            &oracle_uuid,
        ]);

        match result {
            Ok(_) => {
                info!(
                    "Successfully updated NFe identification with ID {}",
                    internal_key
                );
                let updated = self
                    .find_by_id(internal_key)
                    .await?
                    .ok_or(RepositoryError::UpdateFailed)?;

                // Invalidate caches
                if let Err(e) = self.cache.delete(&format!("nfe:{}", internal_key)).await {
                    error!("Failed to invalidate item cache: {}", e);
                }
                if let Err(e) = self.cache.delete("nfe:list:*").await {
                    error!("Failed to invalidate list cache: {}", e);
                }

                Ok(updated)
            }
            Err(e) => {
                error!("Failed to update NFe identification: {}", e);
                Err(RepositoryError::OracleError(e))
            }
        }
    }

    #[instrument(skip(self), fields(internal_key = %internal_key))]
    pub async fn delete(&self, internal_key: &str) -> Result<(), RepositoryError> {
        info!("Deleting NFe identification with ID {}", internal_key);

        // Parse the UUID to ensure it's valid
        let uuid = Uuid::parse_str(internal_key)
            .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

        // Format UUID for Oracle HEXTORAW (remove hyphens)
        let oracle_uuid = uuid.to_string().replace('-', "");

        let sql = "DELETE FROM nfe_identifications WHERE INTERNALKEY = HEXTORAW(:1)";

        let mut stmt = self.conn.statement(sql).build()?;
        let result = stmt.execute(&[&oracle_uuid]);

        match result {
            Ok(_) => {
                info!(
                    "Successfully deleted NFe identification with ID {}",
                    internal_key
                );

                // Invalidate caches
                if let Err(e) = self.cache.delete(&format!("nfe:{}", internal_key)).await {
                    error!("Failed to invalidate item cache: {}", e);
                }
                if let Err(e) = self.cache.delete("nfe:list:*").await {
                    error!("Failed to invalidate list cache: {}", e);
                }

                Ok(())
            }
            Err(e) => {
                error!("Failed to delete NFe identification: {}", e);
                Err(RepositoryError::OracleError(e))
            }
        }
    }
}
