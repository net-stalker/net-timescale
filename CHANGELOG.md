# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate

<!-- [START AUTO UPDATE] -->
<!-- Please keep comment here to allow auto-update -->
## [0.1.0-0b04ec4] - 2024-03-06

NS-116/add-token-verification: added token verification using net-token-verifier (#45)

* added token verification using net-token-verifier
<!-- [END AUTO UPDATE] -->
## [0.1.0-6c5a81f] - 2024-02-28

NS-114/testing-and-fixing-errors: `bug fiixing` (#44)

* fixed a bug with setting bytes filters for bandwidth per protocol
## [0.1.0-04310c7] - 2024-02-26

Ns 108/add filters to nbpp query: added filters for network bandwidth per protocol (#43)

* added filters into query for nbpp
## [0.1.0-8f77890] - 2024-02-24

NS-84/network-bandwidth-per-protocol (#42)

* Add network-bandwidth-per-protocol carcase

* Add continuous aggregate

* Add requester

* Added placeholders for bpp query and updated ca due to filters requirements
## [0.1.0-51656f6] - 2024-02-23

Ns 107/add filters to ng query: added filters for network graph request (#41)

* added filters, updated queries, now we have only one query for links
## [0.1.0-44eb3cc] - 2024-02-23

Ns 106/add filter options to bpe query: filters for network bandwidth per endpoint (#40)

* updated bpe query by adding filters info and updated ca

* updated query by adding filters
## [0.1.0-285a0dd] - 2024-02-23

Ns 103/add filters into chart queries: filters for network bandwidth (#39)

* updated network bandwidth query by adding filters options
## [0.1.0-fcd678a] - 2024-02-19

Ns 97/update deps to cratesio: removed deps from net-registry and added from crates.io (#37)

* updated deps in net-reporter

* updated deps in net-inserter
## [0.1.0-33a0c58] - 2024-02-19

Ns 96/module for network overview filters (#38)

* updated ca creation

* added query, added filters to report manager

* updated sql query for network overview filters. Changed `COALESCE(lhs.total_bytes, rhs.total_bytes, 0) as total_bytes` to `GREATEST(lhs.total_bytes, rhs.total_bytes, 0) as total_bytes`
## [0.1.0-5d2c8ae] - 2024-02-06

Feature/NS-19/async-net-timescale (#36)

* Refactor net-timescale, it is net-inserter now, move everything related to data querying into new net-reporter component (module)

* Move everything related to docker into `docker` directory, automated running timescaledb and timescaledb migrations
## [0.1.0-718629f] - 2023-12-07

Feature/cu 86939c8zv: persistence overview dashboard filters service (#34)

*  added overview dashboard filters persistence and repository modules
## [0.1.0-010ad64] - 2023-11-08

feature/CU-86932w486: updated net-timescale-api version after last refactor (#32)

* updated net-timescale-api version after CU-86930959c
## [0.1.0-06d0177] - 2023-10-25

Feature/cu 8692xggka: implemented queries for bandwidth per endpoint chart (#29)

* added persistence bandwidth per endpoint module
## [0.1.0-c9c6064] - 2023-10-25

Feature/cu 8692vgg0t: implemented dashboard handler (#27)

* implemented dashboard handler for managing numerous chart requests
## [0.1.0-2d6cd0b] - 2023-10-19

Feature/cu 8692y0v0u: reorganized creating continuous aggregates (#31)

* feature/CU-8692y0v0u: split data_aggregate to network_graph_aggregate and bandwidth_per_endpoint_aggregate. Added async trait for continuous aggregates creating refresh policies for them.
## [0.1.0-69f7616] - 2023-10-13

Feature/cu 8692ucf3x: updated graph queries (#26)

* modified getting address pairs and modified mock sqlx::FromRow structures
## [0.1.0-39c4be6] - 2023-10-13

feature/CU-8692vg5mr: fixed liquibase error (#28)

* fixed liquibase error

* fixed tests, wrong version of net-timescale-api was used

* fixed crate versions
## [0.1.0-4d90f83] - 2023-09-29

feature/CU-8692u9pjy: updated continuous aggregate (#25)

* feature/CU-8692u9pjy: updated continuous aggregate, added index for binary_data field, updated methods for creating continuous aggregate
## [0.1.0-49ade13] - 2023-09-28

Feature/cu 8692ngce0: changed `Arc` to `Rc` in `timescale.rs` (#20)

* feature/CU-8692ngce0: started changing Arc to Rc where necessary

* feature/CU-8692ngce0: updated from Arc to rc

* feature/CU-8692ngce0: fixed deps

* feature/CU-8692ngce0: modified net-transport version
## [0.1.0-daf7991] - 2023-09-26

Feature/cu 8692q9mcc: added features from net-monitor:1ae67514 (#23)

* feature/CU-8692q9mcc: deteleted net-timescale-api from cargo workspace, added as a dep from net-api/net-timescale-api

* feature/CU-8692q9mcc: updated migrations, updated code to make it compatible with a new net-timesacle-api

* feature/CU-8692q9mcc: started moving net-monitor code to net-timesacle

* feature/CU-8692q9mcc: added all features from net-monitor:1ae67514

* feature/CU-8692q9mcc: fixed wrong insert method

* feature/CU-8692q9mcc: finished importing changes from net-monitor, tested

* feature/CU-8692q9mcc: fixed quering net-agent-id

* feature/CU-8692q9mcc: removed installing cargo check tools from ci
## [0.1.0-c57c14f] - 2023-09-25

feature/CU-8692te2td: updated ci scripts (#24)

* feature/CU-8692te2td: updated ci scripts, changed owner's credentials to the bot's ones
## [0.1.0-ef54243] - 2023-09-12

Feature/cu 8692p3qy6: refactored a graph structures and added aggregator (#22)

### refactored network graph structures

### feature/CU-8692p3qy6: changed agent_id to aggregator

### feature/CU-8692p3qy6: prepared CHANGELOG.md
## [0.1.0-9ba8c24] - 2023-09-04

Feature/cu 8692nj1c6 (#19)

* feature/CU-8692nj1c6: changed ci flows a little. Removed repositroy dispatch due to using json for sending content to another ci

* feature/CU-8692nj1c6: change test email to secret owner email

* feature/CU-8692nj1c6: removed owner token to prevent calling ci again

* feature/CU-8692nj1c6: polished prepare-changelog-for-update.yml

* feature/CU-8692nj1c6: updated update-changelog.yml

* feature/CU-8692nj1c6: fixed syntax error
## [0.1.0] - 2023-09-04

### Added
Initial change log
