# Metrics exposed

## Process health

| Name | Description |
| ---- | ----------- |
| `fdb_client_timestamp` | Client timestamp (`.client.timestamp` of `status json`) of the last status successfully fetched and parsed |
| `fdb_exporter_parsing_error_count` | Number of parsing errors: a `status json` which could not be deserialized into the models |
| `fdb_exporter_status_not_found_count` | Number of times the status key (`\xff\xff/status/json`) was not found |
| `fdb_exporter_fdb_error_count` | Number of FoundationDB errors (`fdb` feature only) |
| `fdb_exporter_fdb_binding_error_count` | Number of FoundationDB binding errors (`fdb` feature only) |

The `fdb_exporter_*` counters are cumulative over the process lifetime and are
registered lazily on their first increment, so they are absent from the output
until the corresponding error has happened once. The two `fdb_exporter_fdb_*`
counters only exist when the crate is built with the `fdb` feature, which is
always the case for the exporter binary. The binary exits on a binding error
instead of counting it, so `fdb_exporter_fdb_binding_error_count` only moves for
library users calling `to_metrics` on a `FetchError::FdbBinding`.

## FoundationDB

| Name | Description | Labels | Type |
| ---  | ----------- | --- | ---- |
| `fdb_client_coordinator_reachable` | Whether the coordinator is reachable | `["address"]` | GAUGE |
| `fdb_client_coordinators_count` | Number of coordinators registered in client fdb.cluster | `null` | GAUGE |
| `fdb_client_messages_count` | Number of messages available when fetching status | `null` | GAUGE |
| `fdb_client_quorum_reachable` | The quorum of coordinators is reachable | `null` | GAUGE |
| `fdb_client_timestamp` | Client timestamp when last fetched | `null` | GAUGE |
| `fdb_cluster_average_partition_size_bytes` | Average size for a partition in the cluster | `null` | GAUGE |
| `fdb_cluster_backup_paused` | Backup system enabled (0=false) | `null` | GAUGE |
| `fdb_cluster_backup_recent_bytes_per_second` | Rate of bytes sent per second from backup agents | `null` | GAUGE |
| `fdb_cluster_backup_recent_bytes_sent` | Total number of bytes sent recently from backup agents | `null` | GAUGE |
| `fdb_cluster_backup_recent_requests_failed` | Recent number of requests failed to external storage from backup agents | `null` | GAUGE |
| `fdb_cluster_backup_recent_requests_successful` | Recent number of requests done to external storage from backup agents | `null` | GAUGE |
| `fdb_cluster_backup_tag_last_restorable_behind_seconds` | Backup tag information | `["tag"]` | GAUGE |
| `fdb_cluster_backup_tag_last_restorable_version` | Backup tag information | `["tag"]` | GAUGE |
| `fdb_cluster_backup_tag_mutation_log_written_bytes` | Backup tag information | `["tag"]` | GAUGE |
| `fdb_cluster_backup_tag_range_bytes_written` | Backup tag information | `["tag"]` | GAUGE |
| `fdb_cluster_backup_tag_running_backup` | Backup tag information | `["tag"]` | GAUGE |
| `fdb_cluster_backup_tag_running_backup_restorable` | Backup tag information | `["tag"]` | GAUGE |
| `fdb_cluster_backup_workers_running` | Backup system number of agent running in the cluster | `null` | GAUGE |
| `fdb_cluster_backup_workers_total` | Backup system number of agent in the cluster | `null` | GAUGE |
| `fdb_cluster_generation_count` | Number of generations | `null` | GAUGE |
| `fdb_cluster_healthy` | Whether the cluster is healthy or not | `null` | GAUGE |
| `fdb_cluster_latency_commit_seconds` | Time in seconds to commit a transaction | `null` | GAUGE |
| `fdb_cluster_latency_read_seconds` | Time in seconds to read | `null` | GAUGE |
| `fdb_cluster_latency_transaction_start_seconds` | Time in seconds to start a transaction | `null` | GAUGE |
| `fdb_cluster_least_space_log_server_bytes` | Value of the log server with least space available | `null` | GAUGE |
| `fdb_cluster_least_space_storage_server_bytes` | Value of the storage server with least space avaiable | `null` | GAUGE |
| `fdb_cluster_machine_contributing_workers_count` | Number of process workers on the machine | `["address","datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_excluded` | Machine is being excluded of the cluster | `["address","datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_memory_committed_bytes` | Estimated number of bytes of memory not available on the machine | `["address","datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_memory_free_bytes` | Estimated number of bytes of memory that are available on the machine without swapping | `["address","datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_memory_total_bytes` | Estimated number of total physical RAM | `["address","datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_network_received_megabits` | Received megabits | `["address","datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_network_sent_megabits` | Sent megabits | `["address","datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_network_tcp_segment_retransmitted` | Number of TCP segments that have been retransmitted | `["address","datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machines_count` | Number of machines available in the cluster | `null` | GAUGE |
| `fdb_cluster_moving_data_in_flight_bytes` | Data in flight | `null` | GAUGE |
| `fdb_cluster_moving_data_in_queue_bytes` | Data waiting to be transferred | `null` | GAUGE |
| `fdb_cluster_partition_count` | Number of partitions | `null` | GAUGE |
| `fdb_cluster_process_busy` | Busy of the process (value between 0.0 and 1.1) | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_cpu_usage` | Current usage of CPU (between 0 and 1) | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_busy` | Disk is being busy (0.0 to 1.0 value) | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_free_bytes` | Bytes available on the disk used by process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_reads_count` | Number of reads on the disk | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_reads_frequency` | Frequency of reads on the disk | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_reads_sectors` | N/A | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_total_bytes` | Bytes total on the disk used by process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_writes_count` | Number of writes on the disk | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_writes_frequency` | Frequency of writes on the disk | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_writes_sectors` | N/A | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_excluded` | Process is being excluded by the cluster | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_available_bytes` | Available bytes for the current process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_limit_bytes` | Limiting bytes for the current process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_rss_bytes` | N/A | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_unused_allocated_bytes` | N/A | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_used_bytes` | N/A | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_connection_errors_freq` | Frequency of connection errors | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_connections_closed` | Frequency of connection closed | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_connections_established` | Frequency of connection established | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_received_megabits` | Megabits received on network | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_sent_megabits` | Megabits sent on network | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_bytes_queried_counter` | Frequency of write storage server operations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_bytes_queried_hz` | Frequency of write storage server operations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_bytes_queried_roughness` | Frequency of write storage server operations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_count` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_max` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_mean` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_median` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_min` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p25` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p90` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p95` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p99` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p99_9` | Commit batching window size latency  | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_count` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_max` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_mean` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_median` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_min` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_p25` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_p90` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_p95` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_p99` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_latency_p99_9` | GRV proxies latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_count` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_max` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_mean` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_median` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_min` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_p25` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_p90` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_p95` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_p99` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_grv_proxy_batching_p99_9` | GRV proxies commit batching latency | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_count` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_max` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_mean` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_median` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_min` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p25` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p90` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p95` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p99` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p99_9` | Latency for proxies | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_data_lag_seconds` | Lag in seconds of the process role | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_durable_bytes_counter` | Storage and Log input rates durable | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_durable_bytes_hz` | Storage and Log input rates durable | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_durable_bytes_roughness` | Storage and Log input rates durable | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_durable_lag_seconds` | Lag in seconds of data being durable of the process role | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetched_versions_counter` | Frequency of fetched versions in control plane | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetched_versions_hz` | Frequency of fetched versions in control plane | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetched_versions_roughness` | Frequency of fetched versions in control plane | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetches_from_log_counter` | Frequency of fetched data from T logs | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetches_from_log_hz` | Frequency of fetched data from T logs | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetches_from_log_roughness` | Frequency of fetched data from T logs | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_finished_queries_counter` | Number of finished queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_finished_queries_hz` | Number of finished queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_finished_queries_roughness` | Number of finished queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_input_bytes_counter` | Storage and Log Input Rates | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_input_bytes_hz` | Storage and Log Input Rates | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_input_bytes_roughness` | Storage and Log Input Rates | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_keys_queried_counter` | Frequency of read storage server operations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_keys_queried_hz` | Frequency of read storage server operations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_keys_queried_roughness` | Frequency of read storage server operations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_kvstore_available_bytes` | KVStore available bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_kvstore_free_bytes` | KVStore free bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_kvstore_used_bytes` | KVStore used bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_low_priority_queries_counter` | Number of low prio queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_low_priority_queries_hz` | Number of low prio queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_low_priority_queries_roughness` | Number of low prio queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_bytes_counter` | Frequency of mutations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_bytes_hz` | Frequency of mutations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_bytes_roughness` | Frequency of mutations in bytes | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_counter` | Frequency of mutation | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_hz` | Frequency of mutation | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_roughness` | Frequency of mutation | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_disk_available_bytes` | Available bytes in the queue of a process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_disk_free_bytes` | Free bytes in the queue of a process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_disk_total_bytes` | Total bytes in the queue of a process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_disk_used_bytes` | Used bytes in the queue of a process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_max` | Queue of read queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_count` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_max` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_mean` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_median` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_min` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p25` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p90` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p95` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p99` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p99_9` | Latency of read | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_total_queries_counter` | Total number of queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_total_queries_hz` | Total number of queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_total_queries_roughness` | Total number of queries | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_uptime` | Uptime of the process | `["address","class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_processes_roles` | Current number of process running a specific role | `["role"]` | GAUGE |
| `fdb_cluster_state` | Current state of the cluster (see src/status_models/cluster_data.rs) | `null` | GAUGE |
| `fdb_cluster_total_disk_used_bytes` | Total number of bytes used on all disk | `null` | GAUGE |
| `fdb_cluster_total_kv_size_bytes` | Total number of bytes for all key values | `null` | GAUGE |
| `fdb_cluster_wiggle_finished_count` | Number of finished wiggle | `null` | GAUGE |
| `fdb_cluster_wiggle_finished_rounds_count` | Number of finished rounds | `null` | GAUGE |
| `fdb_cluster_wiggle_last_finish` | Timestamp of the last fully finished wiggle | `null` | GAUGE |
| `fdb_cluster_wiggle_last_round_finish` | Timestamp of the last fully finished round | `null` | GAUGE |
| `fdb_cluster_wiggle_last_round_start` | Timestamp of the start of last round | `null` | GAUGE |
| `fdb_cluster_wiggle_last_start` | Timestamp of the start of last wiggle | `null` | GAUGE |
| `fdb_cluster_wiggle_servers_count` | Current number of storage servers being wiggle | `null` | GAUGE |
| `fdb_cluster_wiggle_smoothed_round_seconds` | Seconds elapsed in the current round | `null` | GAUGE |
| `fdb_cluster_wiggle_smoothed_seconds` | Seconds elapsed in the current wiggle | `null` | GAUGE |
| `fdb_database_available` | Database can receive request (0=unavailable) | `null` | GAUGE |
| `fdb_database_healthy` | Database healthiness (0=unhealthy) | `null` | GAUGE |
| `fdb_qos_batch_transactions_per_second_limit` | Number of batch transactions the cluster allows per second | `null` | GAUGE |
| `fdb_qos_limiting_data_lag_storage_server_seconds` | Lag of the limiting storage server | `null` | GAUGE |
| `fdb_qos_limiting_data_lag_storage_server_versions` | Lag of the limiting storage server | `null` | GAUGE |
| `fdb_qos_limiting_durability_lag_storage_server_seconds` | Durability lag of the limiting storage server | `null` | GAUGE |
| `fdb_qos_limiting_durability_lag_storage_server_versions` | Durability lag of the limiting storage server | `null` | GAUGE |
| `fdb_qos_limiting_queue_storage_server_bytes` | Queue of the storage server limiting the system | `null` | GAUGE |
| `fdb_qos_performance_limited_by_reason` | Reason of the system being limited | `null` | GAUGE |
| `fdb_qos_transactions_per_second_limit` | Number of transactions the cluster allows per second | `null` | GAUGE |
| `fdb_qos_worst_data_lag_storage_server_seconds` | Storage server with the worst queue | `null` | GAUGE |
| `fdb_qos_worst_data_lag_storage_server_versions` | Storage server with the worst queue | `null` | GAUGE |
| `fdb_qos_worst_durability_lag_storage_server_seconds` | Storage server with the worst durability queue | `null` | GAUGE |
| `fdb_qos_worst_durability_lag_storage_server_versions` | Storage server with the worst durability queue | `null` | GAUGE |
| `fdb_qos_worst_queue_log_server_bytes` | Worst queue of log server in bytes | `null` | GAUGE |
| `fdb_qos_worst_queue_storage_server_bytes` | Worst queue of storage server | `null` | GAUGE |


## How to generate

1. Run `docker-compose.yml` with `docker compose up -d`
2. Ensure `curl localhost:9090` outputs Metrics
3. Download [`prom2json`](https://github.com/prometheus/prom2json) on your system
4. Ensure you got `jq` on your system
5. Execute jq request below
6. Rewrite the whole table with output data

```
./prom2json http://localhost:9090 | jq 'sort_by(.name) | .[] | @text"| `\(.name)` | \(.help) | `\((.metrics[0]?.labels? | try (keys | join(", ")) catch ""))` | \(.type) |"' -r
```
