# Safe database migrations during rolling release rollback

ENSAMBRIA deploys a new immutable application release while the previous release may
still be running. Database changes therefore **must remain compatible with both the
new and immediately previous application release** until the new release has passed
its health checks and the rollback window has closed.

Use an expand/contract sequence:

1. **Expand:** add nullable columns, tables, indexes, or new enum values in a way that
   the previous release ignores safely. Backfill separately and in bounded batches.
2. **Dual compatibility:** deploy application code that can read the old and new
   representation. During a transition, write both forms if required.
3. **Switch:** after the new release is healthy, move reads and writes to the new form.
4. **Contract:** remove obsolete columns, constraints, indexes, or representations
   only in a later, separately reviewed release after the rollback window is closed.

Automatic staging deployment rejects destructive SQL such as DROP, RENAME,
TRUNCATE, and in-place column type/nullability changes. Do not bypass this guard
to make a deployment pass. Split the change into compatible expand and later
contract migrations instead.

The deployment rollback restores application binaries and restarts all runtime
services. It deliberately does not reverse PostgreSQL migrations: a blind
database rollback can destroy data or make the still-running previous release fail.
