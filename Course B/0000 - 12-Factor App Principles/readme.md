1\. Codebase

-   Single version-controlled repository: Store all application code in one codebase, tracked using version control (e.g., Git).

-   Multiple deploys from one codebase: Deploy to different environments (e.g., development, staging, production) using the same codebase, distinguished only by configuration.

-   Avoid multiple codebases: One app should not have multiple codebases, as this leads to synchronization issues and complicates testing.

* * * *

2\. Dependencies

-   Explicit declaration: List all dependencies in a manifest file (e.g., requirements.txt for Python).

-   Isolate dependencies: Use isolated environments (e.g., virtual environments) to install dependencies, avoiding reliance on system-wide packages.

-   Simplify setup: Explicit declaration ensures new developers can easily set up the app consistently.

* * * *

3\. Config

-   Store in the environment: Keep configuration (e.g., database connection strings, API keys) in environment variables, not hardcoded in the code.

-   Strict separation: Configuration must be separate from the codebase to support multiple deploys from one codebase.

-   Security test: You should be able to open-source your codebase without exposing credentials.

-   Local development: Use a local configuration file (e.g., excluded via .gitignore) for development, distinct from production settings.

* * * *

4\. Backing Services

-   Treat as attached resources: Services consumed over the network (e.g., databases, caching systems, SMTP services) should be loosely coupled.

-   Interchangeable: Swap between local and third-party services (e.g., a local database to a managed cloud database) by changing configuration, not code.

-   Example: An authentication service like PropelAuth can be integrated as a backing service without altering the app's codebase.

* * * *

5\. Build, Release, Run

-   Three distinct stages:
    1.  Build: Convert the codebase into an executable package, including dependency installation.

    2.  Release: Combine the build with the specific configuration for a deployment.

    3.  Run: Execute the app in the target environment.

-   Strict separation: No code changes should occur at runtime; all modifications must go through the build stage again.

* * * *

6\. Processes

-   Stateless design: The app should not store persistent state; all data requiring persistence goes into a stateful backing service (e.g., a database).

-   Benefit: If the app restarts, it resumes without data loss, relying on the backing service.

* * * *

7\. Port Binding

-   Expose via ports: Services should bind to specific ports (e.g., MySQL on port 3306) to handle requests.

-   Consistency: Ports remain consistent across environments (e.g., localhost or production), while IP addresses or domains may vary.

* * * *

8\. Concurrency

-   Scale out with processes: Use multiple independent processes (e.g., web processes, worker processes) to handle concurrency, rather than threading within a single process.

-   Horizontal scaling: Add more processes (e.g., behind a load balancer) to manage increased demand, as opposed to vertical scaling (adding CPU/memory to one machine).

* * * *

9\. Disposability

-   Fast startup: Processes should start quickly to enable rapid scaling and deployment.

-   Graceful shutdown: Shut down cleanly by completing current requests, refusing new ones, and terminating connections properly.

-   Elastic scaling: Disposable processes support scaling up or down without disruption.

* * * *

10\. Dev/Prod Parity

-   Similar environments: Keep development, staging, and production environments as identical as possible (e.g., same backing services).

-   Continuous deployment: Deploy code frequently to minimize differences between environments and reduce bugs.

-   Avoid divergence: Using different services (e.g., SQLite in dev, MySQL in prod) increases risk.

* * * *

11\. Logs

-   Event streams: Treat logs as continuous, timestamped streams, not static files.

-   Write to stdout: Each process should send logs to standard output (stdout), not manage log files itself.

-   Production handling: Capture logs in production with tools like Fluentd, Splunk, or Hadoop for analysis.

* * * *

12\. Admin Processes

-   One-off tasks: Run administrative or management tasks (e.g., database migrations) as standalone processes.

-   Identical environment: Execute these tasks in the same environment as the app's regular processes, using the same codebase to avoid synchronization issues.