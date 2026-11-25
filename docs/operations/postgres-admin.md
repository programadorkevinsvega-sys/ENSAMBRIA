# PostgreSQL administrative operations

PostgreSQL role, schema, ownership, membership, and privilege administration is
separate from application deployment. The normal CORE staging deploy continues
to deploy application artifacts and run the existing CORE/BUSINESS migrations;
it never provisions PostgreSQL roles or schemas.

## Explicit PR commands

Post exactly one of these comments on an open PR targeting `main` in this
repository:

- `/ensambria postgres verify` — read-only catalog audit.
- `/ensambria postgres provision` — idempotent role/schema bootstrap followed
  by the bootstrap SQL's catalog assertions.

Only repository collaborators with `admin` or `maintain` permission may run
the commands. Workflow code and SQL are checked out from trusted `main`, never
from the PR head. The `core-staging` GitHub Environment can require reviewer
approval before secrets are released.

The runner joins the existing Tailscale network and connects with
`DEPLOY_SSH_KEY` to the PostgreSQL VM (SSH user `ubuntu`, host
`CORE_STAGING_POSTGRES_HOST`, default `postgres`). On that VM it runs
`sudo -n docker exec` against container `crm-postgres`, database
`ensambria`. The VM must authorize that SSH key and passwordless sudo for
Docker. No application VM hop is involved.

Neither command changes LOGIN passwords, writes application secrets, edits the
application `.env`, deploys services, or runs application migrations. Any
password rotation must be a separately reviewed operation.

## Required GitHub configuration

Use the existing `core-staging` Environment and secrets:
`DEPLOY_SSH_KEY`, `TS_OAUTH_CLIENT_ID`, and `TS_OAUTH_SECRET`.
Set `CORE_STAGING_POSTGRES_HOST` to the PostgreSQL VM's Tailscale DNS name if
it is not `postgres`. Configure Environment protection reviewers if an
additional approval is required.

The runner tag `tag:core-staging-deployer` must be permitted to reach the
PostgreSQL VM, and the SSH key's matching public key must be authorized for
`ubuntu` on that VM. That account must be able to run the Docker command with
`sudo -n`.
