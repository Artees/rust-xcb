initSidebarItems({"constant":[["XCB_SYNC_ALARM",""],["XCB_SYNC_ALARMSTATE_ACTIVE",""],["XCB_SYNC_ALARMSTATE_DESTROYED",""],["XCB_SYNC_ALARMSTATE_INACTIVE",""],["XCB_SYNC_ALARM_NOTIFY",""],["XCB_SYNC_AWAIT",""],["XCB_SYNC_AWAIT_FENCE",""],["XCB_SYNC_CA_COUNTER",""],["XCB_SYNC_CA_DELTA",""],["XCB_SYNC_CA_EVENTS",""],["XCB_SYNC_CA_TEST_TYPE",""],["XCB_SYNC_CA_VALUE",""],["XCB_SYNC_CA_VALUE_TYPE",""],["XCB_SYNC_CHANGE_ALARM",""],["XCB_SYNC_CHANGE_COUNTER",""],["XCB_SYNC_COUNTER",""],["XCB_SYNC_COUNTER_NOTIFY",""],["XCB_SYNC_CREATE_ALARM",""],["XCB_SYNC_CREATE_COUNTER",""],["XCB_SYNC_CREATE_FENCE",""],["XCB_SYNC_DESTROY_ALARM",""],["XCB_SYNC_DESTROY_COUNTER",""],["XCB_SYNC_DESTROY_FENCE",""],["XCB_SYNC_GET_PRIORITY",""],["XCB_SYNC_INITIALIZE",""],["XCB_SYNC_LIST_SYSTEM_COUNTERS",""],["XCB_SYNC_MAJOR_VERSION",""],["XCB_SYNC_MINOR_VERSION",""],["XCB_SYNC_QUERY_ALARM",""],["XCB_SYNC_QUERY_COUNTER",""],["XCB_SYNC_QUERY_FENCE",""],["XCB_SYNC_RESET_FENCE",""],["XCB_SYNC_SET_COUNTER",""],["XCB_SYNC_SET_PRIORITY",""],["XCB_SYNC_TESTTYPE_NEGATIVE_COMPARISON",""],["XCB_SYNC_TESTTYPE_NEGATIVE_TRANSITION",""],["XCB_SYNC_TESTTYPE_POSITIVE_COMPARISON",""],["XCB_SYNC_TESTTYPE_POSITIVE_TRANSITION",""],["XCB_SYNC_TRIGGER_FENCE",""],["XCB_SYNC_VALUETYPE_ABSOLUTE",""],["XCB_SYNC_VALUETYPE_RELATIVE",""]],"fn":[["xcb_sync_alarm_end",""],["xcb_sync_alarm_next",""],["xcb_sync_await",""],["xcb_sync_await_checked",""],["xcb_sync_await_fence",""],["xcb_sync_await_fence_checked",""],["xcb_sync_change_alarm",""],["xcb_sync_change_alarm_checked",""],["xcb_sync_change_counter",""],["xcb_sync_change_counter_checked",""],["xcb_sync_counter_end",""],["xcb_sync_counter_next",""],["xcb_sync_create_alarm",""],["xcb_sync_create_alarm_checked",""],["xcb_sync_create_counter",""],["xcb_sync_create_counter_checked",""],["xcb_sync_create_fence",""],["xcb_sync_create_fence_checked",""],["xcb_sync_destroy_alarm",""],["xcb_sync_destroy_alarm_checked",""],["xcb_sync_destroy_counter",""],["xcb_sync_destroy_counter_checked",""],["xcb_sync_destroy_fence",""],["xcb_sync_destroy_fence_checked",""],["xcb_sync_fence_end",""],["xcb_sync_fence_next",""],["xcb_sync_get_priority",""],["xcb_sync_get_priority_reply","the returned value must be freed by the caller using libc::free()."],["xcb_sync_get_priority_unchecked",""],["xcb_sync_initialize",""],["xcb_sync_initialize_reply","the returned value must be freed by the caller using libc::free()."],["xcb_sync_initialize_unchecked",""],["xcb_sync_int64_end",""],["xcb_sync_int64_next",""],["xcb_sync_list_system_counters",""],["xcb_sync_list_system_counters_counters_iterator",""],["xcb_sync_list_system_counters_counters_length",""],["xcb_sync_list_system_counters_reply","the returned value must be freed by the caller using libc::free()."],["xcb_sync_list_system_counters_unchecked",""],["xcb_sync_query_alarm",""],["xcb_sync_query_alarm_reply","the returned value must be freed by the caller using libc::free()."],["xcb_sync_query_alarm_unchecked",""],["xcb_sync_query_counter",""],["xcb_sync_query_counter_reply","the returned value must be freed by the caller using libc::free()."],["xcb_sync_query_counter_unchecked",""],["xcb_sync_query_fence",""],["xcb_sync_query_fence_reply","the returned value must be freed by the caller using libc::free()."],["xcb_sync_query_fence_unchecked",""],["xcb_sync_reset_fence",""],["xcb_sync_reset_fence_checked",""],["xcb_sync_set_counter",""],["xcb_sync_set_counter_checked",""],["xcb_sync_set_priority",""],["xcb_sync_set_priority_checked",""],["xcb_sync_systemcounter_end",""],["xcb_sync_systemcounter_name",""],["xcb_sync_systemcounter_name_end",""],["xcb_sync_systemcounter_name_length",""],["xcb_sync_systemcounter_next",""],["xcb_sync_trigger_end",""],["xcb_sync_trigger_fence",""],["xcb_sync_trigger_fence_checked",""],["xcb_sync_trigger_next",""],["xcb_sync_waitcondition_end",""],["xcb_sync_waitcondition_next",""]],"static":[["xcb_sync_id",""]],"struct":[["xcb_sync_alarm_error_t",""],["xcb_sync_alarm_iterator_t",""],["xcb_sync_alarm_notify_event_t",""],["xcb_sync_await_fence_request_t",""],["xcb_sync_await_request_t",""],["xcb_sync_change_alarm_request_t",""],["xcb_sync_change_alarm_value_list_t",""],["xcb_sync_change_counter_request_t",""],["xcb_sync_counter_error_t",""],["xcb_sync_counter_iterator_t",""],["xcb_sync_counter_notify_event_t",""],["xcb_sync_create_alarm_request_t",""],["xcb_sync_create_alarm_value_list_t",""],["xcb_sync_create_counter_request_t",""],["xcb_sync_create_fence_request_t",""],["xcb_sync_destroy_alarm_request_t",""],["xcb_sync_destroy_counter_request_t",""],["xcb_sync_destroy_fence_request_t",""],["xcb_sync_fence_iterator_t",""],["xcb_sync_get_priority_cookie_t",""],["xcb_sync_get_priority_reply_t",""],["xcb_sync_get_priority_request_t",""],["xcb_sync_initialize_cookie_t",""],["xcb_sync_initialize_reply_t",""],["xcb_sync_initialize_request_t",""],["xcb_sync_int64_iterator_t",""],["xcb_sync_int64_t",""],["xcb_sync_list_system_counters_cookie_t",""],["xcb_sync_list_system_counters_reply_t",""],["xcb_sync_list_system_counters_request_t",""],["xcb_sync_query_alarm_cookie_t",""],["xcb_sync_query_alarm_reply_t",""],["xcb_sync_query_alarm_request_t",""],["xcb_sync_query_counter_cookie_t",""],["xcb_sync_query_counter_reply_t",""],["xcb_sync_query_counter_request_t",""],["xcb_sync_query_fence_cookie_t",""],["xcb_sync_query_fence_reply_t",""],["xcb_sync_query_fence_request_t",""],["xcb_sync_reset_fence_request_t",""],["xcb_sync_set_counter_request_t",""],["xcb_sync_set_priority_request_t",""],["xcb_sync_systemcounter_iterator_t",""],["xcb_sync_systemcounter_t",""],["xcb_sync_trigger_fence_request_t",""],["xcb_sync_trigger_iterator_t",""],["xcb_sync_trigger_t",""],["xcb_sync_waitcondition_iterator_t",""],["xcb_sync_waitcondition_t",""]],"type":[["xcb_sync_alarm_t",""],["xcb_sync_alarmstate_t",""],["xcb_sync_ca_t",""],["xcb_sync_counter_t",""],["xcb_sync_fence_t",""],["xcb_sync_testtype_t",""],["xcb_sync_valuetype_t",""]]});