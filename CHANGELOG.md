# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate

<!-- [START AUTO UPDATE] -->
<!-- Please keep comment here to allow auto-update -->
## [0.1.0-c9c6064] - 2023-10-25

Feature/cu 8692vgg0t: implemented dashboard handler (#27)

* implemented dashboard handler for managing numerous chart requests
<!-- [END AUTO UPDATE] -->
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
