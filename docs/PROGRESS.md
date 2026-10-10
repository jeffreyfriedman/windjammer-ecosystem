# Progress

Weekly build health for seed apps and packages.

| Week | Date | Apps / packages green | `extern fn` | Crates via interop | Compiler issues | Notes |
|------|------|----------------------|-------------|--------------------|-----------------|-------|
| 2 | 2026-10-10 | deepen `wj-router`/`wj-cli-args`/`wj-hash` | 0 | 0 | private CARGO_HOME | `require_params`, `require_named_flag`, `require_prefix`; tip **19+26+15** green |
| 2 | 2026-10-09 | deepen `wj-uuid`/`wj-event`/`wj-cookie` | 0 | 0 | private CARGO_HOME | `require_max`, `require_listeners`, `require_cookies`; tip **32+24+26** green |
| 2 | 2026-10-09 | deepen `wj-uuid`/`wj-url`/`wj-event` | 0 | 0 | private CARGO_HOME | `require_v5`, `require_fragment`, `require_pending`; tip **31+28+23** green |
| 2 | 2026-10-09 | deepen `wj-uuid`/`wj-url`/`wj-event` | 0 | 0 | private CARGO_HOME | `require_canonical`, `require_port`, `require_listener`; tip **30+27+22** green |
| 2 | 2026-10-09 | deepen `wj-uuid`/`wj-url`/`wj-jwt` | 0 | 0 | private CARGO_HOME | `require_v1`, `require_query`, `require_expired`; tip **29+26+15** green |
| 2 | 2026-10-09 | deepen `wj-path`/`wj-cli-args`/`wj-csv` | 0 | 0 | private CARGO_HOME | `require_leading`, `require_help`, `require_rows`; tip **30+25+18** green |
| 2 | 2026-10-09 | deepen `wj-path`/`wj-cli-args`/`wj-yaml` | 0 | 0 | private CARGO_HOME | `require_trailing`, `require_any_flag`, `require_empty`; tip **29+24+27** green |
| 2 | 2026-10-09 | deepen `wj-path`/`wj-cli-args`/`wj-template` | 0 | 0 | private CARGO_HOME | `require_dot_dot`, `require_positionals`, `require_placeholders`; tip **28+23+25** green |
| 2 | 2026-10-09 | deepen `wj-path`/`wj-cli-args`/`wj-template` | 0 | 0 | private CARGO_HOME | `require_empty_path`, `require_only_program`, `require_missing`; tip **27+22+24** green |
| 2 | 2026-10-09 | deepen `wj-path`/`wj-event`/`wj-cli-args` | 0 | 0 | private CARGO_HOME | `require_dot`, `require_empty`, `require_empty_args`; tip **26+21+21** green |
| 2 | 2026-10-09 | deepen `wj-timefmt`/`wj-path`/`wj-regex` | 0 | 0 | private CARGO_HOME | `require_equal`, `require_root`, `require_invalid`; tip **36+25+17** green |
| 2 | 2026-10-09 | deepen `wj-timefmt`/`wj-semver`/`wj-mime` | 0 | 0 | private CARGO_HOME | `require_after`, `require_compatible`, `require_octet`; tip **35+27+35** green |
| 2 | 2026-10-09 | deepen `wj-timefmt`/`wj-config`/`wj-semver` | 0 | 0 | private CARGO_HOME | `require_before`, `require_blank`, `require_equal`; tip **34+18+26** green |
| 2 | 2026-10-09 | deepen `wj-mime`/`wj-dotenv`/`wj-migrate` | 0 | 0 | private CARGO_HOME | `require_text`, `require_blank`, `require_pending`; tip **34+18+24** green |
| 2 | 2026-10-09 | deepen `wj-rate-limit`/`wj-timefmt`/`wj-semver` | 0 | 0 | private CARGO_HOME | `require_fresh`, `require_same_year`, `require_newer`; tip **23+33+25** green |
| 2 | 2026-10-09 | deepen `wj-timefmt`/`wj-json-util`/`wj-mime` | 0 | 0 | private CARGO_HOME | `require_same_month`, `require_empty_json`, `require_png`; tip **32+29+33** green |
| 2 | 2026-10-09 | deepen `wj-mime`/`wj-semver`/`wj-migrate` | 0 | 0 | private CARGO_HOME | `require_jpeg`, `require_older`, `require_empty_migrations`; tip **32+24+23** green |
| 2 | 2026-10-09 | deepen `wj-timefmt`/`wj-cors`/`wj-mime` | 0 | 0 | private CARGO_HOME | `require_same_day`, `require_empty_allow`, `require_javascript`; tip **31+22+31** green |
| 2 | 2026-10-09 | deepen `wj-headers`/`wj-rate-limit`/`wj-json-util` | 0 | 0 | private CARGO_HOME | `require_same_origin`, `require_zero_retry`, `require_empty_object`; tip **26+22+28** green |
| 2 | 2026-10-09 | deepen `wj-hash`/`wj-glob`/`wj-cookie` | 0 | 0 | private CARGO_HOME | `require_empty_hash`, `require_candidate`, `require_none`; tip **14+27+25** green |
| 2 | 2026-10-09 | deepen `wj-fs-walk`/`wj-multipart`/`wj-router` | 0 | 0 | private CARGO_HOME | `require_hidden`, `require_empty`, `require_empty_params`; tip **16+20+18** green |
| 2 | 2026-10-09 | deepen `wj-mime`/`wj-cookie`/`wj-cron` | 0 | 0 | private CARGO_HOME | `require_plain`, `require_lax`, `require_wildcard_field`; tip **30+24+38** green |
| 2 | 2026-10-09 | deepen `wj-headers`/`wj-duration`/`wj-semver` | 0 | 0 | private CARGO_HOME | `require_no_referrer`, `require_nonzero`, `require_zero_version`; tip **25+29+23** green |
| 2 | 2026-10-09 | deepen `wj-mime`/`wj-inflect`/`wj-json-util` | 0 | 0 | private CARGO_HOME | `require_xml`, `require_blank`, `require_null`; tip **29+29+27** green |
| 2 | 2026-10-09 | deepen `wj-rate-limit`/`wj-cors`/`wj-semver` | 0 | 0 | private CARGO_HOME | `require_at_limit`, `require_wildcard_origin`, `require_zero_major`; tip **21+21+22** green |
| 2 | 2026-10-09 | deepen `wj-json-util`/`wj-timefmt`/`wj-base64` | 0 | 0 | private CARGO_HOME | `require_empty_array`, `require_leap`, `require_bearer`; tip **26+30+18** green |
| 2 | 2026-10-09 | deepen `wj-mime`/`wj-inflect`/`wj-duration` | 0 | 0 | private CARGO_HOME | `require_css`, `require_title`, `require_at_most`; tip **28+28+28** green |
| 2 | 2026-10-09 | deepen `wj-headers`/`wj-retry`/`wj-compress` | 0 | 0 | private CARGO_HOME | `require_frame_deny`, `require_default_backoff`, `require_identity_encoding`; tip **24+21+23** green |
| 2 | 2026-10-09 | deepen `wj-duration`/`wj-cron`/`wj-cookie` | 0 | 0 | private CARGO_HOME | `require_at_least`, `require_every_minute`, `require_empty`; tip **27+37+23** green |
| 2 | 2026-10-09 | deepen `wj-log`/`wj-mime`/`wj-inflect` | 0 | 0 | private CARGO_HOME | `require_trace`, `require_video`, `require_constant`; tip **22+27+27** green |
| 2 | 2026-10-09 | deepen `wj-log`/`wj-mime`/`wj-inflect` | 0 | 0 | private CARGO_HOME | `require_debug`, `require_html`, `require_pascal`; tip **21+26+26** green |
| 2 | 2026-10-09 | deepen `wj-log`/`wj-mime`/`wj-inflect` | 0 | 0 | private CARGO_HOME | `require_info`, `require_json`, `require_camel`; tip **20+25+25** green |
| 2 | 2026-10-09 | deepen `wj-form-parse`/`wj-scheduler`/`wj-pipeline` | 0 | 0 | private CARGO_HOME | `require_value`, `require_first`, `require_by_id`; tip **12+46+12** green |
| 2 | 2026-10-09 | deepen `wj-find`/`wj-fetch`/`wj-migrate-cli` | 0 | 0 | private CARGO_HOME | `require_last`, `require_retry`, `require_first_pending`; tip **10+41+28** green |
| 2 | 2026-10-09 | deepen `wj-form-parse`/`wj-scheduler`/`wj-migrate-cli` | 0 | 0 | private CARGO_HOME | last field, last expr, `require_last_applied`; tip **11+45+27** green |
| 2 | 2026-10-09 | deepen `wj-fetch`/`wj-hello`/`wj-form-parse` | 0 | 0 | private CARGO_HOME | `require_attempt`, `require_identity`, `require_first`; tip **40+9+10** green |
| 2 | 2026-10-09 | deepen `wj-fetch`/`wj-proxy`/`wj-todo-cli` | 0 | 0 | private CARGO_HOME | `require_info`, `require_unlimited`, `require_empty_todos`; tip **39+34+66** green |
| 2 | 2026-10-09 | deepen `wj-form-parse`/`wj-pipeline`/`wj-sitegen` | 0 | 0 | private CARGO_HOME | `require_empty`, `require_empty_results`, `require_empty_pages`; tip **9+11+29** green |
| 2 | 2026-10-09 | deepen `wj-migrate-cli`/`wj-todo-cli`/`wj-find` | 0 | 0 | private CARGO_HOME | only-pending version, only-pending todo, `require_empty`; tip **26+65+9** green |
| 2 | 2026-10-09 | deepen `wj-fetch`/`wj-hello`/`wj-proxy` | 0 | 0 | private CARGO_HOME | `require_redirect`, `require_smoke`, `require_strip`; tip **38+8+33** green |
| 2 | 2026-10-09 | deepen `wj-scheduler`/`wj-webhook`/`wj-first-hour` | 0 | 0 | private CARGO_HOME | `require_empty_exprs`, `require_dev`, `require_empty_query`; tip **44+20+8** green |
| 2 | 2026-10-09 | deepen `wj-proxy`/`wj-fetch`/`wj-sitegen` | 0 | 0 | private CARGO_HOME | `require_mutating`, `require_client`, `require_html_path`; tip **32+37+28** green |
| 2 | 2026-10-09 | deepen `wj-scheduler`/`wj-fetch`/`wj-proxy` | 0 | 0 | private CARGO_HOME | `require_day_budget`, `require_retryable`, `require_safe_method`; tip **43+36+31** green |
| 2 | 2026-10-09 | deepen `wj-fetch`/`wj-hello`/`wj-first-hour` | 0 | 0 | private CARGO_HOME | `require_server`, `require_zero_major`, `require_default_port`; tip **35+7+7** green |
| 2 | 2026-10-09 | deepen `wj-migrate-cli`/`wj-pipeline`/`wj-form-parse` | 0 | 0 | private CARGO_HOME | `require_caught_up`, `require_only_result`, `require_blank`; tip **25+10+8** green |
| 2 | 2026-10-09 | deepen `wj-todo-cli`/`wj-sitegen`/`wj-webhook` | 0 | 0 | private CARGO_HOME | `require_done`, `require_only_page`, `require_production`; tip **64+27+19** green |
| 2 | 2026-10-09 | deepen `wj-scheduler`/`wj-proxy`/`wj-find` | 0 | 0 | private CARGO_HOME | `require_only`, `require_limited_log`, `require_only_hit`; tip **42+30+8** green |
| 2 | 2026-10-09 | deepen `wj-json-util`/`wj-fs-walk`/`wj-sync` | 0 | 0 | private CARGO_HOME | `require_array`, `require_empty_tree`, `require_closed`; tip **25+15+52** green |
| 2 | 2026-10-09 | deepen `wj-uuid`/`wj-semver`/`wj-dotenv` | 0 | 0 | private CARGO_HOME | `require_nil`, `require_build`, `require_empty`; tip **28+21+17** green |
| 2 | 2026-10-09 | deepen `wj-retry`/`wj-duration`/`wj-config` | 0 | 0 | private CARGO_HOME | `require_exhausted`, `require_negative`, `require_empty`; tip **20+26+17** green |
| 2 | 2026-10-09 | deepen `wj-csv`/`wj-path`/`wj-log` | 0 | 0 | private CARGO_HOME | `require_empty`, `require_hidden`, `require_warn`; tip **17+24+19** green |
| 2 | 2026-10-09 | deepen `wj-template`/`wj-url`/`wj-yaml` | 0 | 0 | private CARGO_HOME | `require_last`, `require_root`, `require_false`; tip **23+25+26** green |
| 2 | 2026-10-09 | deepen `wj-sha`/`wj-base64`/`wj-hash` | 0 | 0 | private CARGO_HOME | `require_upper`, `require_unpadded`, `require_nonempty`; tip **15+17+13** green |
| 2 | 2026-10-09 | deepen `wj-querystring`/`wj-router`/`wj-rate-limit` | 0 | 0 | private CARGO_HOME | `require_empty`, `require_param_pattern`, `require_denied`; tip **25+17+20** green |
| 2 | 2026-10-09 | deepen `wj-cli-args`/`wj-cron`/`wj-compress` | 0 | 0 | private CARGO_HOME | `require_version`, `require_every_hour`, `require_gzip_encoding`; tip **20+36+22** green |
| 2 | 2026-10-09 | deepen `wj-mime`/`wj-cors`/`wj-jwt` | 0 | 0 | private CARGO_HOME | `require_audio`, `require_wildcard`, `require_bearer`; tip **24+20+14** green |
| 2 | 2026-10-08 | deepen `wj-http-client`/`wj-multipart`/`wj-cookie` | 0 | 0 | private CARGO_HOME | `require_error`, `require_text`, `require_strict`; tip **12+19+22** green |
| 2 | 2026-10-08 | deepen `wj-event`/`wj-headers`/`wj-regex` | 0 | 0 | private CARGO_HOME | `require_idle`, `require_nosniff`, `require_none`; tip **20+23+16** green |
| 2 | 2026-10-08 | deepen `wj-timefmt`/`wj-toml`/`wj-migrate` | 0 | 0 | private CARGO_HOME | `require_midnight`, `require_empty`, `require_migrations`; tip **29+27+22** green |
| 2 | 2026-10-08 | deepen `wj-inflect`/`wj-glob`/`wj-validate` | 0 | 0 | private CARGO_HOME | `require_kebab`, `require_meta`, `require_https`; tip **24+26+37** green |
| 2 | 2026-10-08 | deepen `wj-yaml`/`wj-csv`/`wj-path` | 0 | 0 | private CARGO_HOME | `require_true`, `require_cell`, `require_stem`; tip **25+16+23** green |
| 2 | 2026-10-08 | deepen `wj-log`/`wj-retry`/`wj-duration` | 0 | 0 | private CARGO_HOME | `require_error`, `require_first`, `require_zero`; tip **18+19+25** green |
| 2 | 2026-10-08 | deepen `wj-config`/`wj-uuid`/`wj-semver` | 0 | 0 | private CARGO_HOME | `require_value`, `require_v7`, `require_prerelease`; tip **16+27+20** green |
| 2 | 2026-10-08 | deepen `wj-sync` | 0 | 0 | private CARGO_HOME | `require_open`, `require_idle`, `shared_map_require`; tip **51** green |
| 2 | 2026-10-08 | deepen `wj-dotenv`/`wj-fs-walk`/`wj-json-util` | 0 | 0 | private CARGO_HOME | `require_value`, `require_dirs`, `require_object`; tip **16+14+24** green |
| 2 | 2026-10-08 | deepen `wj-hash`/`wj-template`/`wj-url` | 0 | 0 | private CARGO_HOME | `require_min_cost`, `require_placeholder`, `require_http`; tip **12+22+24** green |
| 2 | 2026-10-08 | deepen `wj-sha`/`wj-base64`/`wj-rate-limit` | 0 | 0 | private CARGO_HOME | `require_lower`, `require_padded`, `require_positive_window`; tip **14+16+19** green |
| 2 | 2026-10-08 | deepen `wj-compress`/`wj-querystring`/`wj-router` | 0 | 0 | private CARGO_HOME | `require_identity`, `require_pairs`, `require_static`; tip **21+24+16** green |
| 2 | 2026-10-08 | deepen `wj-jwt`/`wj-cli-args`/`wj-cron` | 0 | 0 | private CARGO_HOME | `require_unexpired`, `require_program`, `require_every_day`; tip **13+19+35** green |
| 2 | 2026-10-08 | deepen `wj-cookie`/`wj-mime`/`wj-cors` | 0 | 0 | private CARGO_HOME | `require_http_only`, `require_image`, `require_preflight`; tip **21+23+19** green |
| 2 | 2026-10-08 | deepen `wj-event`/`wj-headers`/`wj-regex` | 0 | 0 | private CARGO_HOME | `require_named`, `require_csp`, `require_exact_count`; tip **19+22+15** green |
| 2 | 2026-10-08 | deepen `wj-inflect`/`wj-timefmt`/`wj-toml` | 0 | 0 | private CARGO_HOME | `require_snake`, `require_zulu`, `require_nonblank`; tip **23+28+26** green |
| 2 | 2026-10-08 | deepen `wj-validate`/`wj-glob`/`wj-migrate` | 0 | 0 | private CARGO_HOME | `only_error`, `require_exact`, `require_name`; tip **36+25+21** green |
| 2 | 2026-10-08 | deepen `wj-yaml`/`wj-csv`/`wj-path` | 0 | 0 | private CARGO_HOME | `require_path`, `require_rectangular`, `require_extension`; tip **24+15+22** green |
| 2 | 2026-10-08 | deepen `wj-log`/`wj-retry`/`wj-duration` | 0 | 0 | private CARGO_HOME; tip `wj` still emits function-local `use std::strings` (P3.742 source gate GREEN) | `require_enabled`, `require_retry`, `require_within`; tip **17+18+24** green |
| 2 | 2026-10-08 | deepen `wj-http-client` | 0 | 0 | private CARGO_HOME | `is_error_status`/`require_body`; tip **11** green |
| 2 | 2026-10-08 | deepen `wj-scheduler`/`wj-proxy`/`wj-migrate-cli` | 0 | 0 | private CARGO_HOME; P3.742 function-local `use std::strings` gate GREEN | `last_expr`/`only_expr`, `require_upstream`/`require_rate_limit`, `only_pending`/`require_pending`; tip **41+29+24** green |
| 2 | 2026-10-08 | deepen `wj-todo-cli`/`wj-sitegen`/`wj-webhook` | 0 | 0 | private CARGO_HOME | `only_pending`/`require_pending`, `only_page`/`require_pages`, `require_secret`/`require_origins`; tip **63+26+18** green |
| 2 | 2026-10-08 | deepen `wj-hello`/`wj-fetch`/`wj-pipeline` | 0 | 0 | private CARGO_HOME | `require_semver`/`is_smoke_identity`, `require_success`/`status_class`, `only_result`/`require_results`; tip **6+34+9** green |
| 2 | 2026-10-08 | deepen `wj-first-hour`/`wj-find`/`wj-form-parse`/`wj-multipart` | 0 | 0 | private CARGO_HOME | `require_id`/`is_empty_query`, `only_hit`/`require_hits`, `require_fields`/`has_blank_value`; empty multipart bodies keep the header separator (`trim_start`); tip **6+7+7+18** green |
| 2 | 2026-10-08 | deepen `wj-semver` | 0 | 0 | private CARGO_HOME | `is_zero_major`/`require_stable`; tip **19** green |
| 2 | 2026-10-08 | deepen `wj-uuid`/`wj-config`/`wj-multipart` | 0 | 0 | private CARGO_HOME | `hyphen_count`/`require_v4`, `is_blank`/`require_entries`, `first_field_name`/`require_parts`; tip **26+15+17** green |
| 2 | 2026-10-07 | deepen `wj-sha`/`wj-base64`/`wj-rate-limit` | 0 | 0 | private CARGO_HOME | `require_match`/`is_lower_digest`, `require_basic`/`is_padded`, `require_allowed`/`is_positive_window`; tip **13+15+18** green |
| 2 | 2026-10-07 | deepen `wj-hash`/`wj-toml`/`wj-template` | 0 | 0 | private CARGO_HOME | `require_cost`/`is_min_cost`, `require_singleton`/`require_keys`, `last_placeholder`/`require_plain`; tip **11+25+21** green |
| 2 | 2026-10-07 | deepen `wj-cookie`/`wj-mime`/`wj-cors` | 0 | 0 | private CARGO_HOME | `same_site_of`/`is_none_same_site`/`require_secure`, `jpeg`/`is_jpeg`/`content_type_jpeg`, `require_origins`/`allow_origin_of`; tip **20+22+18** green |
| 2 | 2026-10-07 | deepen `wj-dotenv`/`wj-fs-walk`/`wj-json-util` | 0 | 0 | private CARGO_HOME | `is_blank`/`require_nonempty`, `basename_of`/`require_files`, `require_valid`/`is_empty_json`; tip **15+13+23** green |
| 2 | 2026-10-07 | deepen `wj-jwt`/`wj-cli-args`/`wj-cron` | 0 | 0 | private CARGO_HOME | `email_of`/`require_email`/`is_expired`, `program_name`/`require_no_help`, `every_day`/`is_every_day`/`require_cron`; tip **13+18+34** green |
| 2 | 2026-10-07 | deepen `wj-compress`/`wj-querystring`/`wj-router` | 0 | 0 | private CARGO_HOME | `require_gzip`/`require_min_bytes`, `last_value`/`require_singleton`, `first_param_name`/`require_match`; tip **20+23+15** green |
| 2 | 2026-10-07 | deepen `wj-event`/`wj-regex`/`wj-headers` | 0 | 0 | private CARGO_HOME | `has_payload`/`is_named`/`require_payload`, `exactly_n_matches`/`require_valid_pattern`, `hsts_max_age_of`/`content_type_of`/`require_hsts`; tip **18+14+21** green |
| 2 | 2026-10-07 | deepen `wj-log`/`wj-retry`/`wj-validate` | 0 | 0 | private CARGO_HOME | `same_severity`/`is_known_level`, `is_first_attempt`/`next_attempt`/`has_retries_left`, `last_error`/`require_clean`; tip **16+17+35** green |
| 2 | 2026-10-07 | deepen `wj-csv`/`wj-url`/`wj-yaml` | 0 | 0 | private CARGO_HOME | `last_row`/`is_rectangular`, `scheme_of`/`host_of`/`require_https`, `require_valid`; tip **14+23+23** green |
| 2 | 2026-10-07 | deepen `wj-timefmt`/`wj-inflect`/`wj-glob` | 0 | 0 | private CARGO_HOME | `minute_of`/`second_of`/`is_midnight`, `is_blank`/`word_count`/`require_slug`, `has_meta`/`require_match`; tip **27+22+24** green |
| 2 | 2026-10-07 | deepen `wj-migrate`/`wj-duration`/`wj-path` | 0 | 0 | private CARGO_HOME; notes/auth HashMap `"${v}"` still tip RED (P3.730) | `has_migrations`/`earliest_version`/`name_of_version`, `is_at_least`/`is_at_most`/`require_nonneg`, `is_hidden`/`require_relative`; tip **20+23+21** green |
| 2 | 2026-10-07 | deepen `wj-scheduler`/`wj-proxy` | 0 | 0 | private CARGO_HOME; notes-api tip RED (P3.678 regress — HashMap get `"${v}"` → bare `&String`) | `has_exprs`/`first_expr`/`require_exprs`, `upstream_of`/`rate_limit_of`/`log_max_of`/`is_limited_log`; tip **40+28** green; notes deepen deferred |
| 2 | 2026-10-07 | deepen `wj-sitegen`/`wj-webhook`/`wj-migrate-cli` | 0 | 0 | private CARGO_HOME | `has_pages`/`is_empty_pages`/`require_markdown_path`, `secret_of`/`is_production_secret`/`origin_count`/`max_id_len_of`, `first_pending`/`last_applied`/`total_versions`; tip **25+17+23** green |
| 2 | 2026-10-07 | deepen `wj-hello`/`wj-form-parse`/`wj-fetch` | 0 | 0 | private CARGO_HOME | `is_zero_major`/`matches_identity`/`require_crate`, `is_empty_fields`/`first_field_name`/`last_field_name`/`has_value`, `is_informational_status`/`is_redirect_status`/`is_server_error_status`; tip **5+6+33** green |
| 2 | 2026-10-07 | deepen `wj-find`/`wj-pipeline`/`wj-todo-cli` | 0 | 0 | private CARGO_HOME | `last_hit`/`require_first_hit`, `min_result_value`/`has_results`, `has_pending`/`has_done`/`total_count`; tip **6+8+62** green |
| 2 | 2026-10-07 | deepen `wj-first-hour` | 0 | 0 | private CARGO_HOME; GitHub push ISE (local commits pending) | `card_id`/`card_data_path`/`card_query`/`has_id`; tip **5** green |
| 2 | 2026-10-07 | deepen `wj-semver` | 0 | 0 | private CARGO_HOME; GitHub push ISE (local commits pending) | `prerelease_of`/`build_of`/`is_prerelease`; tip **18** green |
| 2 | 2026-10-07 | deepen `wj-rate-limit`/`wj-sha`/`wj-base64` | 0 | 0 | private CARGO_HOME | `window_exhausted`/`is_zero_retry`, `digest_of`/`is_digest`, `needs_padding`/`is_unpadded`/`require_encoded`; tip **17+12+14** green |
| 2 | 2026-10-07 | deepen `wj-validate`/`wj-glob`/`wj-toml` | 0 | 0 | private CARGO_HOME | `first_error`/`has_no_errors`, `exactly_n_matches`/`between_n_matches`, `last_key`/`value_count`/`is_singleton`; tip **34+23+24** green |
| 2 | 2026-10-07 | deepen `wj-compress`/`wj-querystring`/`wj-router` | 0 | 0 | private CARGO_HOME | `accepts_identity`/`prefer_gzip`, `last_key`/`is_singleton`, `has_params`/`is_static_pattern`/`match_or`; tip **19+22+14** green |
| 2 | 2026-10-07 | deepen `wj-path`/`wj-headers`/`wj-cli-args` | 0 | 0 | private CARGO_HOME | `is_dot_dot`/`has_leading_slash`/`require_absolute`, `frame_of`/`csp_of`/`has_hsts`, `has_help_flag`/`has_version_flag`/`require_positional_count`; tip **20+20+17** green |
| 2 | 2026-10-07 | deepen `wj-mime`/`wj-cookie`/`wj-duration` | 0 | 0 | private CARGO_HOME | `png`/`is_png`/`content_type_png`, `has_cookies`/`is_strict_same_site`/`with_same_site`, `is_nonzero`/`outside_ms`/`require_positive`; tip **21+19+22** green |
| 2 | 2026-10-07 | deepen `wj-cron`/`wj-fs-walk`/`wj-retry` | 0 | 0 | private CARGO_HOME | `is_every_hour`/`source_of`, `count_visible_files`/`has_suffix_files`/`is_empty_tree`, `new_backoff`/`is_default_backoff`/`attempts_used`; tip **33+12+16** green |
| 2 | 2026-10-07 | deepen `wj-http-client`/`wj-log`/`wj-jwt` | 0 | 0 | private CARGO_HOME | `status_is_unauthorized`/`status_is_not_found`/`status_of`/`body_of`/`has_body`, `less_severe`/`require_level`, `expires_at`/`has_email`; tip **10+15+12** green |
| 2 | 2026-10-07 | deepen `wj-event`/`wj-regex`/`wj-multipart` | 0 | 0 | private CARGO_HOME | `is_idle`/`event_name`/`event_payload`, `at_least_n_matches`/`require_match`/`is_invalid_pattern`, `has_parts`/`text_count`/`require_file`; tip **17+13+16** green |
| 2 | 2026-10-07 | deepen `wj-hash`/`wj-config`/`wj-template` | 0 | 0 | private CARGO_HOME | `bcrypt_cost`/`is_empty_hash`, `has_keys`/`has_entries`, `is_plain`/`has_missing`/`first_placeholder`; tip **10+14+20** green |
| 2 | 2026-10-07 | deepen `wj-csv`/`wj-url`/`wj-yaml` | 0 | 0 | private CARGO_HOME | `has_rows`/`require_row`/`cell_or`, `without_query`/`with_fragment`/`is_root_path`, `require_int`/`require_bool`; tip **13+22+22** green |
| 2 | 2026-10-07 | deepen `wj-uuid`/`wj-cors`/`wj-json-util` | 0 | 0 | private CARGO_HOME | `with_hyphens`/`require_version`, `is_empty_allow_list`/`has_origins`, `is_nullish`/`is_empty_object`/`is_empty_array`; tip **25+17+22** green |
| 2 | 2026-10-07 | deepen `wj-dotenv`/`wj-timefmt`/`wj-inflect` | 0 | 0 | private CARGO_HOME | `has_keys`/`value_count`, `year_of`/`month_of`/`day_of`/`hour_of`/`is_zulu`, `is_slug`/`is_title_case`; tip **14+26+21** green |
| 2 | 2026-10-07 | deepen `wj-sha`/`wj-base64`/`wj-semver` | 0 | 0 | private CARGO_HOME | `require_etag`, `ensure_padding`, `major_of`/`minor_of`/`patch_of`/`is_zero_version`; tip **11+13+17** green |
| 2 | 2026-10-06 | deepen `wj-validate`/`wj-glob`/`wj-toml` | 0 | 0 | ENOSPC ~5Gi; private CARGO_HOME | `error_count`/`is_valid`, `at_most_n_matches`, `has_keys`/`first_key`; tip **33+22+23** green |
| 2 | 2026-10-06 | deepen `wj-compress`/`wj-querystring`/`wj-router` | 0 | 0 | ENOSPC ~5Gi; private CARGO_HOME | `below_min_bytes`/`negotiate_or_identity`, `has_pairs`/`first_key`, `is_empty_params`/`pattern_param_count`; tip **18+21+13** green |
| 2 | 2026-10-06 | deepen `wj-rate-limit`/`wj-headers`/`wj-cli-args` | 0 | 0 | ENOSPC ~4Gi; private CARGO_HOME | `is_fresh_bucket`/`retry_after_ms_of`/`reset_at_ms_of`, `has_nosniff`/`is_no_referrer`/`referrer_of`, `arg_count`/`is_empty_args`/`only_program_name`; tip **16+19+16** green |
| 2 | 2026-10-06 | deepen `wj-duration`/`wj-mime`/`wj-cookie` | 0 | 0 | ENOSPC ~1–4Gi; private CARGO_HOME | `min_ms`/`max_ms`/`within_ms`, `css`/`xml`/`content_type_*`, `cookie_name`/`cookie_value`/`cookie_path`/`is_lax_same_site`; tip **21+20+18** green |
| 2 | 2026-10-06 | deepen `wj-migrate-cli`/`wj-sitegen`/`wj-path` | 0 | 0 | ENOSPC mid-batch; private CARGO_HOME | `version_count`/`has_pending`/`is_caught_up`, `is_markdown_path`/`is_html_path`/`page_count`, `is_empty_path`/`has_trailing_slash`/`is_dot`; tip **22+24+19** green |
| 2 | 2026-10-06 | deepen `wj-scheduler`/`wj-proxy`/`wj-webhook` | 0 | 0 | ENOSPC ~0.3–2Gi; private CARGO_HOME; Library verify prune Permission denied | `expr_count`/`due_count`/`is_day_budget`, `is_safe_method`/`is_mutating_method`/`has_strip_prefix`/`is_unlimited_log`, `is_dev_secret`/`has_allowed_origins`/`max_event_len_of`; tip **39+27+16** green |
| 2 | 2026-10-06 | deepen `wj-find`/`wj-pipeline`/`wj-first-hour` | 0 | 0 | ENOSPC ~6Gi; private CARGO_HOME | `hit_count`/`has_hits`/`first_hit`, `result_count`/`max_result_value`/`find_result_by_id`, `card_app`/`card_port`/`is_default_port`/`card_summary`; tip **5+7+4** green |
| 2 | 2026-10-06 | deepen `wj-form-parse`/`wj-todo-cli`/`wj-fetch` | 0 | 0 | ENOSPC ~3.5Gi; private CARGO_HOME; package deps via `wj build src` | `field_count`/`has_field`/`field_get_or`/`require_field`, `pending_count`/`done_count`/`find_by_id`/`is_empty_todos`, `is_success_status`/`is_client_error_status`; tip **4+61+32** green |
| 2 | 2026-10-06 | deepen `wj-migrate` + `wj-hello` | 0 | 0 | ENOSPC ~9–10Gi; private CARGO_HOME | `migration_count`/`latest_version`/`pending_count`/`is_pending`; hello `crate_name`/`semver`; tip **19+3** green |
| 2 | 2026-10-06 | deepen `wj-log`/`wj-retry`/`wj-http-client` | 0 | 0 | ENOSPC ~9–10Gi; private CARGO_HOME | `is_trace_level`/`more_severe`, `with_multiplier`/`initial_of`/`multiplier_of`/`max_of`, `status_is_created`/`status_is_no_content`/`body_len`/`is_empty_body`; tip **14+15+9** green |
| 2 | 2026-10-06 | deepen `wj-config`/`wj-cron`/`wj-fs-walk` | 0 | 0 | ENOSPC ~9–10Gi; private CARGO_HOME | `keys`/`values`, `every_hour`/`is_every_minute`, `count_entries`/`has_entries`; tip **13+32+11** green |
| 2 | 2026-10-06 | deepen `wj-hash`/`wj-jwt`/`wj-template` | 0 | 0 | ENOSPC ~9–10Gi; private CARGO_HOME | `is_bcrypt`, `require_jwt`/`subject`/`tenant_slug`, `missing_count`/`require_fully_bound`; tip **9+11+19** green |
| 2 | 2026-10-06 | deepen `wj-event`/`wj-regex`/`wj-multipart` | 0 | 0 | ENOSPC ~9–10Gi; private CARGO_HOME; Library verify prune blocked | `has_listener`/`clear_listeners`, `is_valid_pattern`/`find_or`, `require_field`/`has_file`; tip **16+12+15** green |
| 2 | 2026-10-06 | deepen `wj-uuid`/`wj-cors`/`wj-json-util` | 0 | 0 | ENOSPC ~9–10Gi; private CARGO_HOME | `without_hyphens`/`require_valid`, `require_origin`, `path_require`; tip **24+16+21** green |
| 2 | 2026-10-06 | deepen `wj-sha`/`wj-base64`/`wj-toml` | 0 | 0 | ENOSPC ~9–10Gi; private CARGO_HOME | `hex_upper`/`require_digest`, `has_padding`/`strip_padding`, `merge`; tip **10+12+22** green |
| 2 | 2026-10-06 | deepen `wj-inflect`/`wj-glob`/`wj-csv` | 0 | 0 | ENOSPC ~8–9Gi; private CARGO_HOME; Library verify prune blocked | `is_camel_case`/`is_pascal_case`, `last_match`/`at_least_n_matches`, `require_column`/`get_row`; tip **20+21+12** green |
| 2 | 2026-10-06 | deepen `wj-querystring`/`wj-rate-limit`/`wj-headers` | 0 | 0 | ENOSPC ~9Gi; private CARGO_HOME | `require`, `is_at_limit`/`limit_of`/`window_ms_of`, `without_hsts`/`is_same_origin_frame`; tip **20+15+18** green |
| 2 | 2026-10-06 | deepen `wj-router`/`wj-cli-args`/`wj-compress` | 0 | 0 | ENOSPC ~9Gi; private CARGO_HOME | `param_names`/`require_param`, `has_positionals`/`long_flag_count`, `content_encoding_identity`(+header); tip **12+15+17** green |
| 2 | 2026-10-06 | deepen `wj-cookie`/`wj-mime`/`wj-duration` | 0 | 0 | ENOSPC ~9Gi; private CARGO_HOME | `require_cookie`, `is_plain`/`javascript`/`is_javascript`, `sub_ms`/`abs_ms`/`is_negative`; tip **17+19+20** green |
| 2 | 2026-10-06 | deepen `wj-url`/`wj-yaml`/`wj-path` | 0 | 0 | ENOSPC ~8–9Gi; private CARGO_HOME | `has_query`/`without_fragment`, `is_empty`/`require_str`, `without_trailing_slash`/`is_root`; tip **21+21+18** green |
| 2 | 2026-10-06 | deepen `wj-dotenv`/`wj-semver`/`wj-timefmt` | 0 | 0 | ENOSPC ~8–9Gi; private CARGO_HOME | `merge`, `clear_build`/`is_stable`, `is_same_month`/`is_same_year`; tip **13+16+25** green |
| 2 | 2026-10-06 | deepen `wj-validate` | 0 | 0 | private CARGO_HOME | `require_nonneg`; tip **32** green |
| 2 | 2026-10-06 | deepen `wj-log`/`wj-retry` | 0 | 0 | private CARGO_HOME | `is_info_level`, `with_initial_ms`; tip **13+14** green |
| 2 | 2026-10-06 | deepen `wj-headers`/`wj-uuid`/`wj-cors` | 0 | 0 | private CARGO_HOME | `with_referrer`/`with_hsts`, `is_v1`/`is_v5`, `allowed_origin_count`; tip **17+23+15** green |
| 2 | 2026-10-06 | deepen `wj-inflect`/`wj-event`/`wj-multipart` | 0 | 0 | eco `.cargo-home` syn/`cc` corruption → private CARGO_HOME; Part import for empty Vec | `is_constant_case`, `pending_count`, `is_empty`/`file_count`; tip **19+15+14** green |
| 2 | 2026-10-06 | deepen `wj-toml`/`wj-glob`/`wj-csv` | 0 | 0 | — | `require`/`values`, `exactly_one_match`, `column_index`; tip **21+20+11** green |
| 2 | 2026-10-06 | deepen `wj-config`/`wj-cron` | 0 | 0 | — | `get`/`require`/`is_empty`, `every_minute`/`format_cron`/`is_wildcard_field`; tip **12+31** green |
| 2 | 2026-10-06 | deepen `wj-path`/`wj-compress`/`wj-cli-args` | 0 | 0 | ENOSPC ~10–12Gi free; isolated target | `without_extension`/`ensure_trailing_slash`, `is_identity_encoding`/`meets_min_bytes`, `last_positional`/`has_any_long_flag`; tip **17+16+14** green |
| 2 | 2026-10-06 | deepen `wj-duration`/`wj-querystring`/`wj-rate-limit` | 0 | 0 | private `CARGO_HOME` under ENOSPC | `parse_secs_or`/`is_zero`/`add_ms`, `unique_key_count`/`value_count`, `remaining_slots`/`slots_remaining`; tip **19+19+14** green |
| 2 | 2026-10-06 | deepen `wj-template`/`wj-hash`/`wj-jwt` | 0 | 0 | private `CARGO_HOME` under ENOSPC | `unique_placeholder_count`/`is_fully_bound`, `is_bcrypt_prefix`, `looks_like_jwt`; tip **18+8+10** green |
| 2 | 2026-10-06 | deepen `wj-regex`/`wj-http-client` | 0 | 0 | private `CARGO_HOME` under ENOSPC | `has_matches`/`none_match`, `status_is_ok`/`status_is_redirect`/`status_is_informational`; tip **11+8** green |
| 2 | 2026-10-06 | deepen `wj-log`/`wj-dotenv`/`wj-fs-walk` | 0 | 0 | shared `.cargo-home` syn corruption under ENOSPC → private `CARGO_HOME` | `is_warn_level`/`is_debug_level`, `get`/`values`, `count_dirs`/`has_dirs`; tip **12+12+10** green |
| 2 | 2026-10-06 | deepen `wj-router`/`wj-mime`/`wj-cookie` | 0 | 0 | ENOSPC corrupted `.cargo-home` registry (re-fetched src) | `has_param`/`param_count`/`is_param_pattern`, `plain`/`is_css`/`is_xml`, `cookie_names`/`is_secure`/`is_http_only`; tip **11+18+16** green |
| 2 | 2026-10-06 | deepen `wj-json-util`/`wj-base64`/`wj-sha` | 0 | 0 | — | `path_get_or`/`is_objectish`/`is_arrayish`, `looks_like_base64`/`is_encoded`, `digest_equals`/`is_etag`; tip **20+11+9** green |
| 2 | 2026-10-06 | deepen `wj-cors`/`wj-headers`/`wj-uuid` | 0 | 0 | — | `allows_wildcard`/`is_wildcard_origin`/`cors_line_count`, `without_csp`/`with_frame_options`/`is_frame_deny`, `is_v4`/`is_v7`/`is_canonical`; tip **14+16+22** green |
| 2 | 2026-10-06 | deepen `wj-timefmt`/`wj-semver`/`wj-retry` | 0 | 0 | — | `diff_secs`/`sub_secs`/`is_same_day`, `bump_minor`/`bump_major`/`clear_prerelease`, `clamp_delay_ms`/`with_max_ms`; tip **24+14+13** green |
| 2 | 2026-10-06 | deepen `wj-yaml`/`wj-url`/`wj-validate` | 0 | 0 | P3.687 shared-verify contention (isolated target for url) | `get_int_or`/`get_bool_or`/`is_valid`, `query_get_or`/`has_port`/`has_fragment`/`is_http`, `is_email`/`require_positive`/`has_errors`; tip **20+20+31** green |
| 2 | 2026-10-06 | deepen `wj-csv`/`wj-glob`/`wj-toml` | 0 | 0 | P3.687 (shared-verify Doc-tests E0463; isolated target GREEN) | `is_empty`/`get_cell`/`has_column`, `any_match`/`none_match`/`all_match`, `key_count`/`is_empty`/`keys`; tip **10+19+20** green |
| 2 | 2026-10-06 | deepen `wj-inflect`/`wj-event`/`wj-multipart` | 0 | 0 | — | `constant_case`/`is_snake_case`/`is_kebab_case`, `is_empty`/`has_listeners`/`peek`, `part_count`/`field_names`/`get_field_or`; tip **18+14+13** green |
| 2 | 2026-10-05 | deepen `wj-path`/`wj-cookie`/`wj-compress` | 0 | 0 | — | `is_relative`/`change_extension`/`ensure_leading_slash`, `get_cookie_or`/`is_empty`/`delete_cookie_header`, `default_min_bytes`/`should_compress_default`/`is_gzip_encoding`; tip **16+14+15** green |
| 2 | 2026-10-05 | deepen `wj-mime`/`wj-headers`/`wj-hash` | 0 | 0 | — | `is_octet_stream`/`content_type_json|html`, `header_line_count`/`has_csp`/`with_csp`, `verify_ok`/`require_bcrypt`; tip **17+15+7** green |
| 2 | 2026-10-05 | deepen `wj-duration`/`wj-querystring`/`wj-rate-limit` | 0 | 0 | — | `parse_ms_or`/`is_positive`/`clamp_ms`, `is_empty`/`pair_count`/`keys`, `is_allowed`/`is_denied`/`slots_used`; tip **18+18+13** green |
| 2 | 2026-10-05 | deepen `wj-semver`/`wj-cors`/`wj-jwt` | 0 | 0 | — | `is_older`/`has_prerelease`/`has_build`/`bump_patch`, CORS defaults + `preflight_defaults`, `verify_bearer`; tip **12+12+9** green |
| 2 | 2026-10-05 | deepen `wj-sha`/`wj-base64`/`wj-json-util` | 0 | 0 | — | `digest_hex_len`/`normalize_hex`/`verify_etag`, bearer auth + `decode_or`, `is_valid`/`compact`/`path_exists`; tip **8+10+18** green |
| 2 | 2026-10-05 | deepen `wj-dotenv`/`wj-config`/`wj-cli-args` | 0 | 0 | — (P3.676 bound-arm `has` retained) | `keys`/`key_count`/`is_empty`, config `get_or`/`has`/`key_count`, `nth_positional`/`require_long_flag_value`; tip **11+11+13** green |
| 2 | 2026-10-05 | deepen `wj-retry`/`wj-log`/`wj-fs-walk` | 0 | 0 | P3.681 (count-loop for `count_files`; drop `as int`) | `is_exhausted`/`total_delay_ms`, `is_error_level`/`parse_level_or`, `has_files`; tip **11+11+9** green |
| 2 | 2026-10-05 | deepen `wj-router`/`wj-http-client`/`wj-uuid`/`wj-regex` | 0 | 0 | P3.681 `Ok(vec.len())`→`Result<int,_>` usize (filed; count-loop interim) | `matches_route`/`param_or`, client/server status predicates, `is_version`, `match_count`; tip 0.50.0 **9+7+21+10** green |
| 1 | 2026-08-15 | `wj-hello` | 0 | 0 | Multi-file CLI lib+bin (fixed upstream) | Initial skeleton |
| 2 | 2026-08-16 | `wj-hello`, `wj-dotenv` | 0 | 0 | Flat `lib.wj` + `wj test`, `strings::join` Vec, `fs` AsRef path (fixed upstream) | Idiomatic package seed |
| 2 | 2026-08-16 | + `wj-fetch` (domain + adapters) | 0 | 0 | `json.parse` borrow, HTTP status `u16`, `process::exit` path (fixed upstream) | Wave 1 CLI HTTP GET |
| 2 | 2026-08-17 | + `wj-notes-api` (domain CRUD) | 0 | 0 | HashMap::get non-Copy borrow-break `.cloned()` not `.copied()` (fixed upstream) | Wave 1 REST API seed |
| 2 | 2026-08-18 | + `wj-notes-api` (HTTP adapter) | 0 | 0 | Nested `use std::http::*` emitted `use super::HttpMethod` (fixed upstream) | Hexagonal REST: domain routing + `adapters/http_server.wj` |
| 2 | 2026-08-19 | + `wj-config` | 0 | 0 | HashMap variable-key `.get` codegen (deferred; tests use literal keys) | Layered defaults / file / env |
| 2 | 2026-08-20 | + `wj-log` | 0 | 0 | `use std::log::*` aliased `log_mod`; `error()` homonym skipped `&str` (fixed upstream) | Logging helpers over `std::log` |
| 2 | 2026-08-20 | + `wj-cli-args` | 0 | 0 | string slice/index types (use `std::strings`); for-loop elem → owned helper got `&` (fixed upstream) | Argv helpers until clap interop |
| 2 | 2026-08-20 | + `wj-sitegen` | 0 | 0 | multipass: `pub mod` codegen order; match-arm `&` into owned `string` (fixed upstream) | Wave 2 static site: markdown domain + fs adapters; 10 tests green |
| 2 | 2026-08-21 | + `wj-webhook` | 0 | 0 | | Wave 2 webhook worker: token/sha256 auth, event log, HTTP adapter; 7 tests green |
| 2 | 2026-08-21 | + `wj-http-client` | 0 | 0 | `http::post` body auto-borrow (repro + fixed upstream) | Wave 2 GET/POST helpers over `std::http`; 6 tests green |
| 2 | 2026-08-22 | + `wj-json-util` | 0 | 0 | `json::Value`/`Response` type imports; `json::keys`; owned `get`/`get_index` (fixed upstream) | Wave 2 pretty / path get / deep merge; 11 tests green |
| 2 | 2026-08-22 | + `wj-fs-walk` | 0 | 0 | nested match `for` + `Vec::push` (repro in `windjammer/tests/`; clone codegen fixed upstream) | Wave 2 recursive walk; 6 tests green |
| 2 | 2026-08-22 | + `wj-template` | 0 | 0 | — | Wave 2 `{{key}}` render / strict / missing-keys; 8 tests green |
| 2 | 2026-08-22 | + `wj-uuid` (blocked) | 0 | 0 | `random.range`, `crypto.sha1_bytes`, `time.utc_now` (repros in `windjammer/tests/`) | v1/v4/v5 + validation; 16 tests written, pending stdlib |
| 2 | 2026-08-22 | + `wj-semver` | 0 | 0 | — | Wave 2 parse / compare / format; 6 tests green |
| 2 | 2026-08-22 | + `wj-url` | 0 | 0 | user `join` vs `strings.join` name clash (renamed `join_url`; repro queued) | Wave 2 parse / format / join_url / query; 11 tests green |
| 2 | 2026-08-22 | + `wj-base64` (blocked) | 0 | 0 | `encoding.base64_encode_string` / `decode_string` not in runtime exports (repro queued) | Idiomatic API written; 6 tests pending stdlib |
| 2 | 2026-08-22 | + `wj-retry` | 0 | 0 | — | Wave 2 exponential backoff helpers; 4 tests green |
| 2 | 2026-08-22 | + `wj-sha` | 0 | 0 | — | Wave 2 SHA-256 hex over `std::crypto`; 4 tests green |
| 2 | 2026-08-22 | + `wj-cors` | 0 | 0 | — | Wave 2 CORS allow-list / preflight helpers; 7 tests green |
| 2 | 2026-08-22 | + `wj-router` | 0 | 0 | — | Wave 2 `:param` path match; 7 tests green |
| 2 | 2026-08-22 | + `wj-path` | 0 | 0 | — | Pareto P0 path helpers (`join_path` / normalize / basename / dirname / extname); 10 tests green |
| 2 | 2026-08-22 | + `wj-cookie` | 1 | 0 | `HashMap.get("lit")` after Result match; `for-in` HashMap post-loop drop (repros queued) | Cookie / Set-Cookie parse+format; 8 tests green |
| 2 | 2026-08-22 | + `wj-duration` | 0 | 0 | — | Parse/format `1h30m` ↔ ms; 12 tests green |
| 2 | 2026-08-23 | + `wj-validate` | 2 | 0 | read-only helper ownership; `substring` int→usize (repros queued) | Field validators; 16 tests green |
| 2 | 2026-08-23 | + `wj-glob` | 1 | 0 | loop reuse read-only `pattern` in `filter` (repro queued) | `*`/`?` segment globs; 9 tests green |
| 2 | 2026-08-23 | + `wj-timefmt` | 0 | 0 | — | Pure RFC3339 Zulu parse/format; 6 tests green |
| 2 | 2026-08-23 | + `wj-mime` | 0 | 0 | `std::mime` wiring (repro queued); avoid module `const string` (codegen `&str`) | Pure WJ mime lookup + predicates; 12 tests green |
| 2 | 2026-08-23 | docs | — | — | — | Added `CROSS_ECOSYSTEM_TOP100.md` (npm/PyPI/crates/Go gap analysis) |
| 2 | 2026-08-23 | + `wj-yaml` | 2 | 0 | recursive owned Vec call-site borrow; Vec[int] index (repros queued) | Pure WJ YAML subset + path getters; 12 tests green |
| 2 | 2026-08-23 | + `wj-rate-limit` | 0 | 0 | — | Fixed-window buckets + Retry-After / X-RateLimit-*; 8 tests green |
| 2 | 2026-08-23 | + `wj-headers` | 0 | 0 | — | Helmet-style secure defaults; 8 tests green |
| 2 | 2026-08-23 | + `wj-migrate` | 1 | 0 | `vec.len() - int` loop bound usize/i64 (repro queued) | Parse/sort/pending migrations + schema SQL; 10 tests green |
| 2 | 2026-08-23 | + `wj-inflect` | 0 | 0 | — | snake/camel/pascal/slugify; 8 tests green |
| 2 | 2026-08-23 | `wj-migrate` smoke | 0 | 0 | — | Postgres smoke via `platform-db`/`wj_migrate_smoke` + `scripts/smoke_postgres.sh` |
| 2 | 2026-08-23 | + `wj-todo-cli` | 2 | 0 | string ordinal compare; HashMap.insert if-arm Option (repros queued) | Hexagonal todo CLI; 14 tests green |
| 2 | 2026-08-23 | stdlib focus | — | — | New `bug_std_*` for yaml/jwt/uuid/path/csv/db/time/sha256 | Added `STDLIB_GRADUATION.md` + `windjammer/tests/STDLIB_ADOPTION_QUEUE.md` |
| 2 | 2026-08-24 | + `wj-proxy` | 0 | 0 | ownership/type codegen in adapters (fixed in `.wj`) | Hexagonal reverse proxy; 9 tests |
| 2 | 2026-08-24 | + `wj-querystring` | 0 | 0 | `Vec::push((key, ""))` empty literal (repro queued) | parse/stringify/get/get_all; 12 tests green |
| 2 | 2026-08-24 | + `wj-event` | 0 | 0 | — | Queue-based event bus + wildcard patterns; 8 tests |
| 2 | 2026-08-25 | + `wj-multipart` | 0 | 0 | reused string after owned call; demoted `&str` auto-borrow (repros queued) | form-data parse/format; 9 tests green |
| 2 | 2026-08-25 | unblock `wj-base64` | 0 | 0 | `Vec<u8>` push int→u8 (repro queued) | 6 tests green over `std::encoding` |
| 2 | 2026-08-25 | unblock `wj-uuid` | 0 | 0 | module const→owned formal (repro queued); `sha1_bytes` now returns `Vec` | 16 tests green |
| 2 | 2026-08-25 | + `wj-jwt` | 0 | 0 | — | HS256 sign/verify over `std::jwt`; 5 tests green |
| 2 | 2026-08-25 | + `wj-csv` | 0 | 0 | `csv.write` via user `fn write` clone-without-borrow (repro queued) | parse via `std::csv`; pure WJ write; 6 tests green |
| 2 | 2026-08-25 | + `wj-toml` | 0 | 0 | — | TOML config subset → flat map; 8 tests green |
| 2 | 2026-09-11 | deepen `wj-toml` | 0 | 0 | — | Single-line arrays → compact JSON; 12 tests green |
| 2 | 2026-08-26 | deepen `wj-timefmt` | 0 | 0 | — | offsets + epoch + compare; 18 tests green |
| 2 | 2026-08-26 | + `wj-hash` | 0 | 0 | — | bcrypt hash/verify over `std::crypto`; 4 tests green |
| 2 | 2026-08-26 | `wj-csv` → std write | 0 | 0 | write-homonym gate green | thin `std::csv` parse/write |
| 2 | 2026-08-26 | + `wj-compress` | 0 | 0 | `while int < strings.len` (repro queued) | Accept-Encoding + gzip codecs; 10 tests green |
| 2 | 2026-08-26 | + `wj-regex` | 0 | 0 | — | Thin `std::regex` wrappers; 9 tests green |
| 2 | 2026-08-27 | deepen `wj-url` | 0 | 0 | demoted `string` reuse / owned formal `&` (repros in `bug_match_none_arm_string_after_split_test`) | `query_has` / `query_remove` / `with_query` + join keeps query/fragment; 15 tests green |
| 2 | 2026-08-27 | `wj-querystring` → `std::encoding` | 0 | 0 | — | `%HH` / form `+` via `url_encode`/`url_decode`; 14 tests green |
| 2 | 2026-08-27 | deepen `wj-glob` | 0 | 0 | — | recursive `**` segment match; 14 tests green |
| 2 | 2026-08-29 | deepen `wj-template` | 0 | 0 | — | `render_with_defaults`, `escape_html`, `render_html`, trimmed placeholders; 13 tests green |
| 2 | 2026-08-29 | + `wj-auth-api` | 0 | 0 | cross-crate owned/borrow forwarder (`bug_app_cross_crate_owned_forwarder_emits_borrow_test`); domain uses `std::crypto`/`jwt`/`compress` for hash/JWT/gzip body until green | Hexagonal auth API: register/login/me + CORS + gzip + HTML welcome; 10 tests green |
| 2 | 2026-08-29 | deepen `wj-notes-api` | 0 | 0 | HashMap field `.get(i64)` auto-borrow (`bug_hashmap_field_get_i64_key_auto_borrow_test`); cross-crate `WindowBucket` Serialize on app struct; demoted string after OPTIONS borrow | Dogfood `wj-router`/`wj-cors`/`wj-headers`/`wj-rate-limit`; health + CORS + 429; 23 tests green |
| 2 | 2026-09-02 | deepen `wj-proxy` | 0 | 0 | — | `PROXY_LOG_MAX` ring buffer trims oldest entries; config + proxy tests; **21/21 tests green** |
| 2 | 2026-09-02 | deepen `wj-proxy` | 0 | 0 | read-only helper + owned `Vec` param → owned `self` (P3.221 repro; stats inlined in `handle_local`) | `DELETE /logs` clears buffer + `cleared` count; `bind_addr` var for `server_serve`; **19/19 tests green** |
| 2 | 2026-08-30 | deepen `wj-proxy` | 0 | 0 | — | hexagonal `domain/config.wj`, `GET /stats`, config + stats tests; 14 tests green |
| 2 | 2026-09-01 | deepen `wj-todo-cli` | 0 | 0 | `Vec::len()` usize vs `int` tuple (workaround: `pending + done`) | `stats` / `stats --json`; `import --merge` run test; `count_todos` in query; **54/54 tests green** |
| 2 | 2026-09-02 | deepen `wj-todo-cli` | 0 | 0 | `match` in `if` without `return` on `Err` arm (E0308); `todos_from_json(body)` emits `&body`; string literal at `rename` call site | `edit <id> <title>` command; `TodoStore::rename`; **60/60 tests green** |
| 2 | 2026-09-01 | deepen `wj-todo-cli` | 0 | 0 | `json.is_array`/`json.len` owned `Value` borrow (P3.214 repro; `[` prefix + index loop workaround) | `import` / `import --merge`; `todos_from_json` + `merge_from`; **48/48 tests green** |
| 2 | 2026-09-01 | deepen `wj-todo-cli` | 0 | 0 | — | `export` / `export --out` / filters; `todos_to_json` in codec; store preserved via snapshot; **42/42 tests green** |
| 2 | 2026-09-01 | deepen `wj-todo-cli` | 0 | 0 | — | `clear` / `clear --done` on `TodoStore`; **35/35 tests green** |
| 2 | 2026-09-01 | deepen `wj-sitegen` | 0 | 0 | — | `robots.txt` via `render_robots` when `SITEGEN_BASE_URL` set; **23/23 tests green** |
| 2 | 2026-09-01 | deepen `wj-sitegen` | 0 | 0 | cross-crate `join_url`/`render` owned→borrow multipass (`bug_multipass_cross_crate_join_url_owned_base_test`, `bug_multipass_cross_crate_render_owned_template_helper_test`) | sitemap + RSS via `domain/feeds.wj`, `SITEGEN_BASE_URL`/`SITEGEN_TITLE`, inline template + `join_page_url`; **21/21 tests green** |
| 2 | 2026-08-30 | deepen `wj-sitegen` | 0 | 0 | app multipass cross-crate `own()` forwarder still emits `&` (`bug_app_multipass_cross_crate_owned_forwarder_module_file_test`) | recursive `*.md` walk, `SITEGEN_SRC`/`SITEGEN_OUT`, `wj-template` escape + layout; 14 tests green |
| 2 | 2026-09-01 | deepen `wj-fetch` | 0 | 0 | — | `-s` / `--silent` body-only stdout; silent + `--output` writes file with no stdout; **28/28 tests green** |
| 2 | 2026-09-02 | deepen `wj-fetch` | 0 | 1 | `use std::net::Request` import not codegen'd (P3.219); adapter still uses `std::http::get` | `--timeout=N` / `FETCH_TIMEOUT_SECS` parsing on `FetchRequest`; **31/31 tests green** |
| 2 | 2026-09-01 | deepen `wj-fetch` | 0 | 0 | `match Some(string)` / `None => ""` E0308 (P3.217 repro; mut reassignment workaround) | `--output` / `-o` write body to file; `resolve_fetch_output` in format; **24/24 tests green** |
| 2 | 2026-08-31 | deepen `wj-fetch` | 0 | 0 | `std::async_runtime` import RED — `wj-retry::pause_ms` uses `std::time` spin-wait | Backoff pause between retries; **20/20 tests green** (wj-retry 5/5) |
| 2 | 2026-08-31 | deepen `wj-todo-cli` | 0 | 0 | owned struct field → fn `string` param emits `.clone()` not borrow | `list --pending`/`--done`, sorted by id, `domain/query.wj`; **28/28 tests green** |
| 2 | 2026-08-30 | deepen `wj-proxy` | 0 | 0 | — | Dogfood `wj-rate-limit` + `wj-headers`; security + rate-limit headers; 11 tests green |
| 2 | 2026-08-31 | deepen `wj-auth-api` | 0 | 0 | `HttpMethod` test/lib split (same as webhook); string public port | hexagonal `domain/config.wj`, internal `HttpMethod` routing, `ServerResponse` helpers; **11/11 tests green** |
| 2 | 2026-08-31 | deepen `wj-notes-api` | 0 | 0 | HashMap i64 `.get` borrow; spurious `use Note;` same-file Serialize; `store.get(id)` → `get(&id)` at api boundary | hexagonal `domain/config.wj` + `note.wj`, `lookup_note`/`fetch_note`, internal `HttpMethod`, `ServerResponse` helpers; **24/24 tests green** |
| 2 | 2026-08-31 | deepen `wj-webhook` (regression) | 0 | 0 | spurious `use BusEventBody;` same-file Serialize (E0255) | split `domain/bus_event.wj`; **15/15 tests green** |
| 2 | 2026-08-30 | deepen `wj-webhook` | 0 | 0 | cross-crate owned forwarder for `wj-validate` literals | Dogfood `wj-cors` + `wj-headers` + `wj-validate`; CORS preflight + field limits; 12 tests green |
| 2 | 2026-08-30 | deepen `wj-migrate` | 0 | 0 | `DirEntry.name()` unwired; cross-module `Connection` emits `.clone()`; `pub mod` at end truncates `lib.rs` | `db_status::status_url` + shared discover; 18 tests green |
| 2 | 2026-08-30 | + `wj-migrate-cli` | 0 | 0 | cross-crate `Vec` borrow blocks `wj-cli-args`; local argv parsing | apply + status CLI; 12 tests green |
| 2 | 2026-08-31 | deepen `wj-migrate-cli` | 0 | 0 | — | `list` subcommand (filesystem-only, dogfoods `discover_migrations`); **17/17 tests green** |
| 2 | 2026-09-01 | deepen `wj-migrate-cli` | 0 | 0 | string var moved to `run_list`/`run_validate` then reused in `teardown_dir` (E0382) — literal path in teardown | `validate` subcommand (filesystem-only, dogfoods `validate_unique_versions`); **21/21 tests green** |
| 2 | 2026-08-31 | deepen `wj-retry` | 0 | 0 | `use std::async_runtime::sleep_ms_blocking` emits `as async` keyword alias (repro filed) | `pause_ms` via `std::time` spin-wait; **5/5 tests green** |

| 2 | 2026-09-03 | + `wj-cron` | 0 | 0 | — | Cron expression parsing + matching (5 fields, `*`, `*/N`, lists, ranges); **14/14 tests green** |
| 2 | 2026-09-03 | deepen `wj-cron` | 0 | 0 | — | `next_run` minute stepper + `CronDateTime` (cross-hour / weekday skip / max look-ahead); **19/19 tests green** |
| 2 | 2026-09-03 | deepen `wj-cron` | 0 | 0 | — | Named aliases (`@hourly`…`@annually`) + range/step (`1-10/2`); **28/28 tests green** |
| 2 | 2026-09-03 | + `wj-scheduler` | 0 | 0 | Vec helper after index → `&Vec` formal + `rest.clone()` E0308 (P3.224 repro; flag scan inlined) | Hexagonal CLI dogfoods `wj-cron`: `check` / `next` / `explain`; **22/22 tests green** |
| 2 | 2026-09-04 | deepen `wj-cron` | 0 | 0 | — | `CronExpr.source` keeps `parse_cron` owned formal; **30/30 tests green** |
| 2 | 2026-09-04 | deepen `wj-scheduler` | 0 | 0 | demoted `&str` ↔ owned `String` call-site (P3.225 / P3.233 repros; `"${expr}"` + direct `matches_cron`) | Crontab `list` / `due`; **37/37 tests green** |
| 2 | 2026-09-04 | deepen `wj-yaml` | 0 | 0 | — | Block scalars `|` (literal) + `>` (folded); **16/16 tests green** |
| 2 | 2026-09-04 | deepen `wj-validate` | 0 | 0 | — | `require_url` + `require_uuid`; **23/23 tests green** |
| 2 | 2026-09-11 | graduate `wj-mime` | 0 | 1 | `std::mime.from_*` charset ≠ APPLICATION_* (P3.243 repro) | Constants via `std::mime`; lookup stays WJ; **12/12 tests green** |
| 2 | 2026-09-11 | graduate `wj-path` | 0 | 0 | — | `join_path`/`basename` → `std::path`; normalize/dirname/extname sugar; **10/10 tests green** |
| 2 | 2026-09-11 | graduate `wj-yaml` | 0 | 1 | `std::yaml.to_json` accepts empty input (P3.244 repro); package pre-checks | `to_json` → `std::yaml`; get_* path sugar; **16/16 tests green** |
| 2 | 2026-09-11 | full graduate `wj-mime` | 0 | 0 | — | Full thin-wrap `std::mime` (lookup + predicates); P3.243 ✅; **12/12 tests green** |
| 2 | 2026-09-11 | full graduate `wj-yaml` | 0 | 0 | — | Drop empty pre-check; `to_json` → `std::yaml` only; P3.244 ✅; **16/16 tests green** |
| 2 | 2026-09-12 | deepen `wj-uuid` | 0 | 0 | tip `i64 & 0xff` → `255_u8` (P3.250 repro) | RFC 9562 **v7** + **NIL**/**MAX**; **20/20** on wj 0.50.0 |
| 2 | 2026-09-12 | deepen `wj-toml` | 0 | 0 | tip `get(&text)` owned formal (P3.254); `"${raw}"` interim (P3.251) | Dotted keys + inline tables; **17/17** on wj 0.50.0 |
| 2 | 2026-09-12 | deepen `wj-validate` | 0 | 0 | — | `require_uuid` accepts v1/v4/v5/v7 + RFC variant; **27/27** |
| 2 | 2026-09-12 | deepen `wj-auth-api` | 0 | 0 | multipass lit→demoted `&str` + `.to_string()` (P3.259); dual-runtime `HttpMethod` in tests | Register assigns **UUID v7** id; JWT `sub`=id; `handle_http` adapter port; **12/12** |
| 2 | 2026-09-12 | deepen `wj-config` | 0 | 0 | — | `from_toml` / `resolve_toml` via `wj-toml`; **7/7** |
| 2 | 2026-09-13 | deepen `wj-auth-api` config | 0 | 0 | HashMap get → demoted `&str` + `.clone()` (P3.261); interim `"${v}"` | `config_from_toml` dogfoods wj-config/wj-toml; **15/15** |
| 2 | 2026-09-13 | deepen `wj-auth-api` cookies + rate-limit | 0 | 0 | tip `wj` (2026-09-13) RED on owned cross-crate formals / notes-api; verified on cargo-bin `wj` 0.50.0 (2026-08-27); P3.262 metadata gate | Cookie session (`wj-cookie`) + `/logout` + fixed-window rate limit (`wj-rate-limit`); **20/20** |
| 2 | 2026-09-13 | deepen `wj-auth-api` security headers | 0 | 0 | — | Dogfood `wj-headers` on JSON / OPTIONS / 429; **23/23** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` validate | 0 | 0 | — | Dogfood `wj-validate` (nonempty/min/max) + config limits; **28/28** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` hash+jwt | 0 | 0 | — | Dogfood `wj-hash`/`wj-jwt`; configurable `tenant_slug` on `/me`; **30/30** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` router | 0 | 0 | — | Dogfood `wj-router` for dispatch + path normalize; **32/32** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` mime | 0 | 0 | — | Dogfood `wj-mime` `Content-Type` on `HttpReply` + adapter; **33/33** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` duration | 0 | 0 | — | Dogfood `wj-duration` for JWT TTL / window (`2h`, `30m`, `2m`); **36/36** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` dotenv | 0 | 0 | — | Dogfood `wj-dotenv` `config_from_dotenv` + key map; **40/40** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` timefmt | 0 | 0 | — | Dogfood `wj-timefmt` `/me.expires_at` RFC3339; **41/41** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` inflect | 0 | 0 | — | Dogfood `wj-inflect` username slugify on register/login; **44/44** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` log | 0 | 0 | — | Dogfood `wj-log` (`log_level` + access lines); **48/48** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-notes-api` validate+mime | 0 | 0 | cargo-bin residual: owned helper → demoted `method: &str` (gate tip GREEN; adapter uses `handle_http`) | Dogfood `wj-validate`/`wj-mime`; title/body limits + JSON `Content-Type`; **29/29** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-notes-api` dotenv | 0 | 0 | cargo-bin mid-match HashMap defer-drop in owned `.get` helper (repro filed); single-map reader like auth | Dogfood `wj-dotenv`/`wj-config::merge` `config_from_dotenv`; **32/32** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-notes-api` log | 0 | 0 | — | Dogfood `wj-log` (`log_level` + `[notes]` access lines); **35/35** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` toml | 0 | 0 | — | Dogfood `wj-config`/`wj-toml` `config_from_toml` (flat + `[limits]`); **39/39** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` duration | 0 | 0 | — | Dogfood `wj-duration` for rate window (`2m`, `30s`); **42/42** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` compress | 0 | 0 | — | Dogfood `wj-compress` gzip negotiate + `Content-Encoding`; **44/44** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` template | 0 | 0 | — | Dogfood `wj-template` `GET /` welcome HTML; **46/46** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` querystring | 0 | 0 | demoted query slice → `query_get` needs `own(query)` on cargo-bin | Dogfood `wj-querystring` `GET /notes?limit=N`; **47/47** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` uuid+timefmt | 0 | 0 | — | Dogfood `wj-uuid` v7 `uid` + `wj-timefmt` RFC3339 `created_at`; **51/51** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` inflect | 0 | 0 | — | Dogfood `wj-inflect` title `slug`; **54/54** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` sha/etag | 0 | 0 | cargo-bin: owned local → demoted `&str` (tip GREEN `bug_demoted_str_formal_owned_local_auto_borrow_test`); notes holds match string in `Vec` so formal stays owned | Dogfood `wj-sha` ETag + If-None-Match 304; **56/56** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` json-util | 0 | 0 | — | Dogfood `wj-json-util` `?pretty=1`; **57/57** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` base64 | 0 | 0 | cargo-bin over-borrows cross-crate `encode` (repro P3.282); use `encode_text` | Dogfood `wj-base64` `?encoding=base64`; **58/58** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` url Location | 0 | 0 | import alias `query_get` steals `wj-url` Borrowed metadata (repro P3.283); alias as `qs_get` | Dogfood `wj-url` `Location` + `public_base_url`; **60/60** on cargo-bin `wj` |
| 2 | 2026-09-14 | deepen `wj-notes-api` regex search | 0 | 0 | owned `Vec<Note>` filter helper demote+clone (repro P3.284); inline filter loop | Dogfood `wj-regex` `GET /notes?q=`; **62/62** on cargo-bin `wj` |
| 2 | 2026-10-04 | deepen `wj-event` | 0 | 0 | — | `queue_len` / `clear_queue` keep listeners; **11/11** tip green |
| 2 | 2026-10-04 | deepen `wj-cli-args` | 0 | 0 | — | `long_flag_value_or` default fallback; **10/10** tip green |
| 2 | 2026-10-04 | deepen `wj-rate-limit` | 0 | 0 | — | `rate_limit_header_lines` for adapters; **10/10** tip green |
| 2 | 2026-10-04 | tip dogfood `wj-todo-cli` | 0 | 0 | P3.642 tip GREEN (20:06) | snapshot first-use clone; **60/60** |
| 2 | 2026-10-04 | deepen `wj-cookie` | 0 | 0 | — | `get_cookie` + `session_cookie` defaults; **10/10** tip green |
| 2 | 2026-10-04 | tip dogfood `wj-notes-api` | 0 | 0 | P3.666 qs_get key `.to_string()` | product `$WJ test` **4 E0308** tip RED |
| 2 | 2026-10-04 | tip dogfood `wj-notes-api` P3.666 | 0 | 0 | — | tip 21:42 bare qs_get keys GREEN (cargo 3/3 + transpile) |
| 2 | 2026-10-04 | deepen `wj-cors` | 0 | 0 | — | `is_preflight_method` + `cors_header_lines`; **9/9** tip green |
| 2 | 2026-10-04 | deepen `wj-headers` | 0 | 0 | — | `security_header_lines` for adapters; **12/12** tip green |
| 2 | 2026-10-04 | deepen `wj-compress` | 0 | 0 | — | `vary_accept_encoding` + `should_compress`; **12/12** tip green |
| 2 | 2026-10-04 | deepen `wj-dotenv` | 0 | 0 | — | `get_or` default lookup; **7/7** tip green |
| 2 | 2026-10-04 | deepen `wj-sha` | 0 | 0 | — | `etag` quoted digest; **5/5** tip green |
| 2 | 2026-10-04 | deepen `wj-mime` | 0 | 0 | — | `content_type` + `is_json`; **14/14** tip green |
| 2 | 2026-10-04 | deepen `wj-semver` | 0 | 0 | — | `is_newer` + `is_compatible`; **8/8** tip green |
| 2 | 2026-10-04 | deepen `wj-glob` | 0 | 0 | — | `first_match` for path lists; tip green |
| 2 | 2026-10-04 | deepen `wj-path` | 0 | 0 | — | `is_absolute` + `has_extension`; **12/12** tip green |
| 2 | 2026-10-04 | deepen `wj-base64` | 0 | 0 | — | `basic_auth_header`; **7/7** tip green |
| 2 | 2026-10-04 | deepen `wj-jwt` | 0 | 0 | — | `bearer_header` + `extract_bearer`; **7/7** tip green |
| 2 | 2026-10-04 | deepen `wj-duration` | 0 | 0 | — | `parse_secs` / `format_secs`; **14/14** tip green |
| 2 | 2026-10-04 | deepen `wj-querystring` | 0 | 0 | — | `set` / `get_or`; **16/16** tip green |
| 2 | 2026-10-04 | deepen `wj-url` | 0 | 0 | — | `origin` / `is_https`; **18/18** tip green |
| 2 | 2026-10-04 | deepen `wj-retry` | 0 | 0 | — | `attempts_remaining` / `default_backoff`; **9/9** tip green |
| 2 | 2026-10-04 | tip dogfood `wj-json-util` P3.669 | 0 | 0 | — | tip 23:43 demote take_field; **15/15** tip green |
| 2 | 2026-10-04 | deepen `wj-inflect` | 0 | 0 | — | `title_case`; **15/15** tip green |
| 2 | 2026-10-04 | deepen `wj-log` | 0 | 0 | — | `level_rank` / `is_enabled`; **9/9** tip green |
| 2 | 2026-10-04 | deepen `wj-hash` | 0 | 0 | — | `looks_like_bcrypt`; **5/5** tip green |
| 2 | 2026-10-04 | deepen `wj-csv` | 0 | 0 | — | `row_count` / `column_count`; **7/7** tip green |
| 2 | 2026-10-05 | deepen `wj-yaml` | 0 | 0 | — | `get_str_or` / `has_path`; **18/18** tip green |
| 2 | 2026-10-05 | deepen `wj-fs-walk` | 0 | 0 | — | `count_files`; **8/8** tip green |
| 2 | 2026-10-05 | deepen `wj-dotenv` | 0 | 1 | P3.676 `Some(_)`→matches!+DEFER DROP | `has`/`require`; has uses bound arm until tip greens |
| 2 | 2026-10-05 | deepen `wj-toml` | 0 | 0 | — | `get_or` / `has`; tip green |
| 2 | 2026-10-05 | deepen `wj-timefmt` | 0 | 0 | — | `is_equal` / `add_secs`; **22/22** tip green |
| 2 | 2026-10-05 | deepen `wj-validate` | 0 | 0 | — | `require_len_range`; **28/28** tip green |
| 2 | 2026-10-05 | deepen `wj-event` | 0 | 0 | — | `has_pending`; **12/12** tip green |
| 2 | 2026-10-05 | tip dogfood P3.671 | 0 | 0 | — | tip 18:38 `query.clone()` (no format!); GREEN |
| 2 | 2026-10-05 | deepen `wj-cookie` | 0 | 1 | P3.676 bound arm until tip greens | `has_cookie` / `cookie_count`; **12/12** tip green |
| 2 | 2026-10-05 | deepen `wj-cli-args` | 0 | 0 | — | `positional_count`; **11/11** tip green |
| 2 | 2026-10-05 | deepen `wj-rate-limit` | 0 | 0 | — | `peek_fixed_window`; **11/11** tip green |
| 2 | 2026-10-05 | deepen `wj-cors` | 0 | 0 | — | `format_cors_headers`; **10/10** tip green |
| 2 | 2026-10-05 | deepen `wj-headers` | 0 | 0 | — | `hsts_enabled`; **13/13** tip green |
| 2 | 2026-10-05 | deepen `wj-duration` | 0 | 0 | — | `ms_to_secs` / `secs_to_ms`; **15/15** tip green |
| 2 | 2026-10-05 | deepen `wj-mime` | 0 | 0 | — | `is_html`; **15/15** tip green |
| 2 | 2026-10-05 | deepen `wj-semver` | 0 | 0 | — | `is_equal`; **9/9** tip green |
| 2 | 2026-10-05 | deepen `wj-path` | 0 | 0 | — | `stem`; **13/13** tip green |
| 2 | 2026-10-05 | deepen `wj-compress` | 0 | 0 | — | `content_encoding_header`; **13/13** tip green |
| 2 | 2026-10-05 | deepen `wj-sha` | 0 | 1 | P3.679 while lit+substring | `looks_like_sha256_hex` (chars); **6/6** tip green |
| 2 | 2026-10-05 | deepen `wj-glob` | 0 | 0 | — | `match_count`; **17/17** tip green |
| 2 | 2026-10-05 | tip dogfood P3.679 | 0 | 0 | — | tip 18:38 `while i < 64_usize` RED; filed |
| 2 | 2026-10-05 | deepen `wj-base64` | 0 | 0 | — | `is_basic_auth_header`; **8/8** tip green |
| 2 | 2026-10-05 | deepen `wj-jwt` | 0 | 0 | — | `is_bearer_header`; **8/8** tip green |
| 2 | 2026-10-05 | deepen `wj-template` | 0 | 0 | — | `placeholder_count`; **17/17** tip green |
| 2 | 2026-10-05 | deepen `wj-multipart` | 0 | 0 | — | `has_field`; **11/11** tip green |

## Weekly checklist

- [ ] No project-local `ffi/` / hand-written `.rs` / `extern fn` in apps
- [ ] CI green on pinned `wj`
- [ ] Changed seeds pass `wj test`
- [ ] [STDLIB_COVERAGE.md](STDLIB_COVERAGE.md) updated when production code gains std usage
- [ ] Compiler bugs get failing tests in `windjammer/tests/`
