use crate::models::nfe_identification::{CreateNFeIdentification, NFeIdentification};
use chrono::{DateTime, NaiveDateTime, Utc};
use oracle::Connection;
use std::fmt;
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
}

impl NFeIdentificationRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub async fn find_all(&self) -> Result<Vec<NFeIdentification>, RepositoryError> {
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
            ORDER BY DHEMI DESC
        "#;

        let mut stmt = self.conn.statement(sql).build()?;
        let rows = stmt.query(&[])?;
        let mut identifications = Vec::new();

        for row_result in rows {
            let row = row_result?;
            let internal_key_hex: String = row.get(0)?;
            let dh_emi_str: String = row.get(7)?;
            let created_at_str: String = row.get(20)?;
            let updated_at_str: String = row.get(21)?;

            let internal_key = Uuid::parse_str(&internal_key_hex)
                .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

            let dh_emi = NaiveDateTime::parse_from_str(&dh_emi_str, "%Y-%m-%d %H:%M:%S%.3f")
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|_| Utc::now());

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
                tp_nf: row.get(8)?,
                id_dest: row.get(9)?,
                c_mun_fg: row.get(10)?,
                tp_imp: row.get(11)?,
                tp_emis: row.get(12)?,
                c_dv: row.get(13)?,
                tp_amb: row.get(14)?,
                fin_nfe: row.get(15)?,
                ind_final: row.get(16)?,
                ind_pres: row.get(17)?,
                proc_emi: row.get(18)?,
                ver_proc: row.get(19)?,
                created_at,
                updated_at,
            };
            identifications.push(identification);
        }

        Ok(identifications)
    }

    pub async fn find_by_id(
        &self,
        internal_key: &str,
    ) -> Result<Option<NFeIdentification>, RepositoryError> {
        let uuid = Uuid::parse_str(internal_key)
            .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

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
        let mut rows = stmt.query(&[&uuid.to_string()])?;

        if let Some(row_result) = rows.next() {
            let row = row_result?;
            let internal_key_hex: String = row.get(0)?;
            let dh_emi_str: String = row.get(7)?;
            let created_at_str: String = row.get(20)?;
            let updated_at_str: String = row.get(21)?;

            let internal_key = Uuid::parse_str(&internal_key_hex)
                .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

            let dh_emi = NaiveDateTime::parse_from_str(&dh_emi_str, "%Y-%m-%d %H:%M:%S%.3f")
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|_| Utc::now());

            let created_at =
                NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now());

            let updated_at =
                NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S%.3f")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(|_| Utc::now());

            Ok(Some(NFeIdentification {
                internal_key: internal_key.to_string(),
                c_uf: row.get(1)?,
                c_nf: row.get(2)?,
                nat_op: row.get(3)?,
                mod_: row.get(4)?,
                serie: row.get(5)?,
                n_nf: row.get(6)?,
                dh_emi,
                tp_nf: row.get(8)?,
                id_dest: row.get(9)?,
                c_mun_fg: row.get(10)?,
                tp_imp: row.get(11)?,
                tp_emis: row.get(12)?,
                c_dv: row.get(13)?,
                tp_amb: row.get(14)?,
                fin_nfe: row.get(15)?,
                ind_final: row.get(16)?,
                ind_pres: row.get(17)?,
                proc_emi: row.get(18)?,
                ver_proc: row.get(19)?,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create(
        &self,
        identification: &CreateNFeIdentification,
    ) -> Result<NFeIdentification, RepositoryError> {
        println!("Creating identification with data: {:?}", identification);

        let internal_key = Uuid::new_v4();
        let internal_key_hex = internal_key.to_string().replace("-", "");
        println!("Generated internal key: {}", internal_key_hex);

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
            )
            VALUES (
                HEXTORAW(:1), :2, :3, :4, :5, :6, :7, TO_TIMESTAMP_TZ(:8, 'YYYY-MM-DD HH24:MI:SS.FF3 TZR'), :9, :10,
                :11, :12, :13, :14, :15, :16, :17, :18, :19, :20
            )
        "#;

        let dh_emi_str = identification
            .dh_emi
            .format("%Y-%m-%d %H:%M:%S%.3f %z")
            .to_string();
        println!("Formatted dh_emi: {}", dh_emi_str);

        let mut stmt = self.conn.statement(sql).build()?;

        // Execute with parameters directly
        match stmt.execute(&[
            &internal_key_hex,
            &identification.c_uf,
            &identification.c_nf,
            &identification.nat_op,
            &identification.mod_,
            &identification.serie,
            &identification.n_nf,
            &dh_emi_str,
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
        ]) {
            Ok(_) => println!("SQL execution successful"),
            Err(e) => {
                eprintln!("SQL execution failed: {:?}", e);
                return Err(RepositoryError::OracleError(e));
            }
        }

        // Now fetch the created record
        let fetch_sql = r#"
            SELECT 
                RAWTOHEX(INTERNALKEY),
                CUF,
                CNF,
                NATOP,
                MOD_,
                SERIE,
                NNF,
                TO_CHAR(DHEMI, 'YYYY-MM-DD HH24:MI:SS.FF3 TZR'),
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
                TO_CHAR(CREATEDAT, 'YYYY-MM-DD HH24:MI:SS.FF3 TZR'),
                TO_CHAR(UPDATEDAT, 'YYYY-MM-DD HH24:MI:SS.FF3 TZR')
            FROM nfe_identifications
            WHERE INTERNALKEY = HEXTORAW(:1)
        "#;

        let mut stmt = self.conn.statement(fetch_sql).build()?;
        let mut rows = stmt.query(&[&internal_key_hex])?;

        if let Some(row_result) = rows.next() {
            let row = row_result?;
            let internal_key_hex: String = row.get(0)?;
            let internal_key = Uuid::parse_str(&internal_key_hex)
                .map_err(|e| RepositoryError::InvalidUuid(e.to_string()))?;

            let dh_emi_str: String = row.get(7)?;
            let dh_emi = DateTime::parse_from_str(&dh_emi_str, "%Y-%m-%d %H:%M:%S%.3f %z")
                .map_err(|_| RepositoryError::InvalidUuid("Invalid date format".to_string()))?
                .with_timezone(&Utc);

            let created_at_str: String = row.get(20)?;
            let created_at = DateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S%.3f %z")
                .map_err(|_| RepositoryError::InvalidUuid("Invalid date format".to_string()))?
                .with_timezone(&Utc);

            let updated_at_str: String = row.get(21)?;
            let updated_at = DateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S%.3f %z")
                .map_err(|_| RepositoryError::InvalidUuid("Invalid date format".to_string()))?
                .with_timezone(&Utc);

            Ok(NFeIdentification {
                internal_key: internal_key.to_string(),
                c_uf: row.get(1)?,
                c_nf: row.get(2)?,
                nat_op: row.get(3)?,
                mod_: row.get(4)?,
                serie: row.get(5)?,
                n_nf: row.get(6)?,
                dh_emi,
                tp_nf: row.get(8)?,
                id_dest: row.get(9)?,
                c_mun_fg: row.get(10)?,
                tp_imp: row.get(11)?,
                tp_emis: row.get(12)?,
                c_dv: row.get(13)?,
                tp_amb: row.get(14)?,
                fin_nfe: row.get(15)?,
                ind_final: row.get(16)?,
                ind_pres: row.get(17)?,
                proc_emi: row.get(18)?,
                ver_proc: row.get(19)?,
                created_at,
                updated_at,
            })
        } else {
            Err(RepositoryError::CreationFailed)
        }
    }

    pub async fn update(
        &self,
        internal_key: &str,
        identification: &NFeIdentification,
    ) -> Result<NFeIdentification, RepositoryError> {
        println!("Starting update for internal_key: {}", internal_key);
        println!("Update data: {:?}", identification);

        let uuid = Uuid::parse_str(internal_key).map_err(|e| {
            println!("Invalid UUID error: {}", e);
            RepositoryError::InvalidUuid(e.to_string())
        })?;

        let uuid_hex = uuid.to_string().replace("-", "");
        println!("Formatted UUID hex: {}", uuid_hex);

        let update_sql = r#"
            UPDATE nfe_identifications
            SET 
                CUF = :1,
                CNF = :2,
                NATOP = :3,
                MOD_ = :4,
                SERIE = :5,
                NNF = :6,
                DHEMI = TO_TIMESTAMP_TZ(:7, 'YYYY-MM-DD HH24:MI:SS.FF3 TZR'),
                TPNF = :8,
                IDDEST = :9,
                CMUNFG = :10,
                TPIMP = :11,
                TPEMIS = :12,
                CDV = :13,
                TPAMB = :14,
                FINNFE = :15,
                INDFINAL = :16,
                INDPRES = :17,
                PROCEMI = :18,
                VERPROC = :19,
                UPDATEDAT = CURRENT_TIMESTAMP
            WHERE INTERNALKEY = HEXTORAW(:20)
        "#;

        let dh_emi_str = identification
            .dh_emi
            .format("%Y-%m-%d %H:%M:%S%.3f %z")
            .to_string();
        println!("Formatted dh_emi: {}", dh_emi_str);

        let mut stmt = self.conn.statement(update_sql).build()?;
        println!("Built update statement");

        match stmt.execute(&[
            &identification.c_uf,
            &identification.c_nf,
            &identification.nat_op,
            &identification.mod_,
            &identification.serie,
            &identification.n_nf,
            &dh_emi_str,
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
            &uuid_hex,
        ]) {
            Ok(_) => println!("Update executed successfully"),
            Err(e) => {
                println!("Update execution failed: {:?}", e);
                return Err(RepositoryError::OracleError(e));
            }
        }

        // Now fetch the updated record
        let fetch_sql = r#"
            SELECT 
                RAWTOHEX(INTERNALKEY),
                CUF,
                CNF,
                NATOP,
                MOD_,
                SERIE,
                NNF,
                TO_CHAR(DHEMI, 'YYYY-MM-DD HH24:MI:SS.FF3 TZR'),
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
                TO_CHAR(CREATEDAT, 'YYYY-MM-DD HH24:MI:SS.FF3 TZR'),
                TO_CHAR(UPDATEDAT, 'YYYY-MM-DD HH24:MI:SS.FF3 TZR')
            FROM nfe_identifications
            WHERE INTERNALKEY = HEXTORAW(:1)
        "#;

        println!("Fetching updated record");
        let mut stmt = self.conn.statement(fetch_sql).build()?;
        let mut rows = stmt.query(&[&uuid_hex])?;

        if let Some(row_result) = rows.next() {
            match row_result {
                Ok(row) => {
                    println!("Successfully fetched updated record");
                    let internal_key_hex: String = row.get(0)?;
                    let internal_key = Uuid::parse_str(&internal_key_hex).map_err(|e| {
                        println!("Invalid UUID error in fetch: {}", e);
                        RepositoryError::InvalidUuid(e.to_string())
                    })?;

                    let dh_emi_str: String = row.get(7)?;
                    let dh_emi = DateTime::parse_from_str(&dh_emi_str, "%Y-%m-%d %H:%M:%S%.3f %z")
                        .map_err(|_| {
                            println!("Invalid date format for dh_emi: {}", dh_emi_str);
                            RepositoryError::InvalidUuid("Invalid date format".to_string())
                        })?
                        .with_timezone(&Utc);

                    let created_at_str: String = row.get(20)?;
                    let created_at =
                        DateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S%.3f %z")
                            .map_err(|_| {
                                println!("Invalid date format for created_at: {}", created_at_str);
                                RepositoryError::InvalidUuid("Invalid date format".to_string())
                            })?
                            .with_timezone(&Utc);

                    let updated_at_str: String = row.get(21)?;
                    let updated_at =
                        DateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S%.3f %z")
                            .map_err(|_| {
                                println!("Invalid date format for updated_at: {}", updated_at_str);
                                RepositoryError::InvalidUuid("Invalid date format".to_string())
                            })?
                            .with_timezone(&Utc);

                    Ok(NFeIdentification {
                        internal_key: internal_key.to_string(),
                        c_uf: row.get(1)?,
                        c_nf: row.get(2)?,
                        nat_op: row.get(3)?,
                        mod_: row.get(4)?,
                        serie: row.get(5)?,
                        n_nf: row.get(6)?,
                        dh_emi,
                        tp_nf: row.get(8)?,
                        id_dest: row.get(9)?,
                        c_mun_fg: row.get(10)?,
                        tp_imp: row.get(11)?,
                        tp_emis: row.get(12)?,
                        c_dv: row.get(13)?,
                        tp_amb: row.get(14)?,
                        fin_nfe: row.get(15)?,
                        ind_final: row.get(16)?,
                        ind_pres: row.get(17)?,
                        proc_emi: row.get(18)?,
                        ver_proc: row.get(19)?,
                        created_at,
                        updated_at,
                    })
                }
                Err(e) => {
                    println!("Error fetching row: {:?}", e);
                    Err(RepositoryError::OracleError(e))
                }
            }
        } else {
            println!("No rows found after update");
            Err(RepositoryError::UpdateFailed)
        }
    }

    pub async fn delete(&self, internal_key: &str) -> Result<(), RepositoryError> {
        println!("Starting delete for internal_key: {}", internal_key);

        let uuid = Uuid::parse_str(internal_key).map_err(|e| {
            println!("Invalid UUID error: {}", e);
            RepositoryError::InvalidUuid(e.to_string())
        })?;

        let uuid_hex = uuid.to_string().replace("-", "");
        println!("Formatted UUID hex: {}", uuid_hex);

        let sql = "DELETE FROM nfe_identifications WHERE INTERNALKEY = HEXTORAW(:1)";
        let mut stmt = self.conn.statement(sql).build()?;

        match stmt.execute(&[&uuid_hex]) {
            Ok(_) => {
                println!("Delete executed successfully");
                Ok(())
            }
            Err(e) => {
                println!("Delete execution failed: {:?}", e);
                Err(RepositoryError::OracleError(e))
            }
        }
    }
}
