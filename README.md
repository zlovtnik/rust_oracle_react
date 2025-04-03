# NFe Management System

A full-stack application for managing NFe (Nota Fiscal Eletrônica) documents, built with Rust and Svelte.

## Project Structure

- `rust_oracle_react/` - Rust backend API
- `svelte-frontend/` - Svelte frontend application

## Prerequisites

- Rust (latest stable version)
- Oracle Database
- Oracle Client (including SQL*Plus)
- Node.js and npm
- Cargo (Rust package manager)

## Backend Setup

1. Make sure Oracle Database is installed and running
2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your Oracle database credentials
```

3. Run database migrations:
```bash
./scripts/setup_db.sh
```

4. Start the backend server:
```bash
cargo run
```

## Frontend Setup

1. Install dependencies:
```bash
cd svelte-frontend
npm install
```

2. Start the development server:
```bash
npm run dev
```

## API Endpoints

- `GET /api/identifications` - List all NFe identifications
- `POST /api/identifications` - Create new NFe identification
- `GET /api/identifications/{id}` - Get specific NFe identification
- `PUT /api/identifications/{id}` - Update NFe identification
- `DELETE /api/identifications/{id}` - Delete NFe identification

## Development

- Backend runs on `http://localhost:8080`
- Frontend runs on `http://localhost:5173`

## License

MIT 