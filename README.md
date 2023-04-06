# BOOK PLANNER

## TODO
- specific-binary rebuild on demand for quicker iteration
- move knative_fn shared tilt func to separate place
- tooling to
  - generate bin targets from cmd / values files
  - copy all wasms to bin dir
- figure out how to split knative functions by path on a shared hostname

- DB Operator
  - Automatically create a DB in the same namespace as the operator deployment
  - CRD for a database definition
  - CRD for defining migrations
    - Have an `index` property so that you can sling them all in at once
    - Have a migrations table where it can diff to ensure integrity