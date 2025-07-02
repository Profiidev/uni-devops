# Dealing with Devops

This is a little poker game deployed to a azure kubernetes cluster.

## Frontend

The frontend is written in Svelte and with SvelteKit and SSR.
It lives in the [frontend](frontend) directory (more information can be found there).

## Backend

The backend is written in Rust and uses the Rocket framework.
It lives in the [backend](backend) directory (more information can be found there).

## Development

To setup the local environment you need a `.env` file in the backend directory an example can be found [here](.env.example).
All required services, backend and frontend (in dev mode) can be started with the [docker-compose](docker-compose.yml) file.

```bash
docker compose up -d
```

## Deployment

The deployment is done using GitHub Actions and Terraform.
The deployment files are in the [terraform](terraform) directory (more information can be found there).
