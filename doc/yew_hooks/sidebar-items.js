window.SIDEBAR_ITEMS = {"enum":[["UseSwipeDirection","Swipe direction."],["UseWebSocketReadyState","The current state of the `WebSocket` connection."]],"fn":[["use_async","This hook returns state and a `run` callback for an async future."],["use_async_with_options","This hook returns state and a `run` callback for an async future with options. See [`use_async`] too."],["use_before_unload","A side-effect hook that shows browser alert when user try to reload or close the page."],["use_bool_toggle","This hook is a simplified [`use_toggle`] to manage boolean toggle state in a function component."],["use_click_away","A hook that triggers a callback when user clicks outside the target element."],["use_clipboard","This hook is used to read from or write to clipboard for text or bytes. e.g. copy plain text or copy `image/png` file to clipboard."],["use_counter","This hook is used to manage counter state in a function component."],["use_debounce","A hook that delays invoking a function until after wait milliseconds have elapsed since the last time the debounced function was invoked."],["use_debounce_effect","A hook that delays calling effect callback until after wait milliseconds have elapsed since the last time effect callback was called."],["use_debounce_effect_with_deps","This hook is similar to [`use_debounce_effect`] but it accepts dependencies."],["use_debounce_state","A hook that delays updating state until after wait milliseconds have elapsed since the last time state was updated."],["use_default","A state hook that returns the default value when state is None."],["use_drag","This hook tracks file, link and copy-paste drags."],["use_drag_with_options","This hook tracks file, link and copy-paste drags. [`use_drag`] hook with options."],["use_drop","This hook tracks file, link and copy-paste drops."],["use_drop_with_options","This hook tracks file, link and copy-paste drops. [`use_drop`] hook with options."],["use_effect_once","A lifecycle hook that runs an effect only once."],["use_effect_update","This hook ignores the first invocation (e.g. on mount). The signature is exactly the same as the [`use_effect`] hook."],["use_effect_update_with_deps","This hook is similar to [`use_effect_update`] but it accepts dependencies. The signature is exactly the same as the [`use_effect_with_deps`] hook."],["use_event","A hook that subscribes a callback to events."],["use_event_with_window","A hook that subscribes a callback to events only for window. If you want to specify an event target, use [`use_event`]."],["use_favicon","A side-effect hook that sets favicon of the page."],["use_geolocation","A sensor hook that tracks user’s geographic location."],["use_geolocation_with_options","A sensor hook that tracks user’s geographic location. See [`use_geolocation`]"],["use_hash","A sensor hook that tracks brower’s location hash value."],["use_infinite_scroll","A sensor hook that tracks infinite scrolling of the element."],["use_interval","A hook that schedules an interval to invoke `callback` every `millis` milliseconds. The interval will be cancelled if `millis` is set to 0."],["use_is_first_mount","A hook returns true if component is just mounted (on first render) and false otherwise."],["use_is_mounted","A hook returns true if component is mounted and false otherwise."],["use_latest","This hook returns the latest immutable ref to state or props."],["use_list","A hook that tracks a list and provides methods to modify it."],["use_local_storage","A side-effect hook that manages a single localStorage key."],["use_location","A sensor hook that tracks brower’s location value."],["use_logger","This hook logs in console as component goes through life-cycles."],["use_logger_eq","This hook logs in console as component goes through life-cycles. Like [`use_logger`] but only logs when `prev_state != next_state`. This requires the props to implement [`PartialEq`]."],["use_map","A hook that tracks a hash map and provides methods to modify it."],["use_measure","A sensor hook that tracks an HTML element’s dimensions using the `ResizeObserver` API."],["use_media","This hook plays video or audio and exposes its controls."],["use_media_with_options","This hook plays video or audio and exposes its controls with options. see [`use_media`]"],["use_mount","A lifecycle hook that calls a function after the component is mounted."],["use_mut_latest","This hook returns the latest mutable ref to state or props."],["use_previous","This hook returns the previous immutable ref to state or props."],["use_queue","A hook that tracks a queue and provides methods to modify it."],["use_raf","An animation hook that forces component to re-render on each `requestAnimationFrame`, returns percentage of time elapsed. `millis` - milliseconds for how long to keep re-rendering component. `delay` — delay in milliseconds after which to start re-rendering component."],["use_raf_state","A state hook that only updates state in the callback of `requestAnimationFrame`."],["use_renders_count","A hook that counts component renders."],["use_scroll","A sensor hook that tracks an HTML element’s scroll position."],["use_scrolling","A sensor hook that tracks whether HTML element is scrolling."],["use_search_param","A sensor hook that tracks brower’s location search param value."],["use_session_storage","A side-effect hook that manages a single sessionStorage key."],["use_set","A hook that tracks a hash set and provides methods to modify it."],["use_size","A sensor hook that tracks an HTML element’s dimensions using the `ResizeObserver` API."],["use_state_ptr_eq","Similar to `use_state_eq`, but check if the two `Rc`s of values point to the same allocation, instead of PartialEq of the values."],["use_swipe","A sensor hook that detects swipe based on TouchEvent."],["use_swipe_with_options","A sensor hook that detects swipe based on TouchEvent with options. If you want to detect for window, pass `NodeRef::default()` to param `node`."],["use_swipe_with_window","A sensor hook that detects swipe based on TouchEvent for window. See [`use_swipe`]."],["use_throttle","A hook that throttles invoking a function, the function is only executed once every `millis`."],["use_throttle_effect","A hook that throttles calling effect callback, it is only called once every `millis`."],["use_throttle_effect_with_deps","This hook is similar to [`use_throttle_effect`] but it accepts dependencies."],["use_throttle_state","A hook that throttles updating state, the state is only updated once every `millis`."],["use_timeout","A hook that schedules a timeout to invoke `callback` in `millis` milliseconds from now. The timeout will be cancelled if `millis` is set to 0 or `cancel()` is called."],["use_title","A side-effect hook that sets title of the page and restore previous title when unmount."],["use_toggle","This hook is used to manage toggle state in a function component."],["use_unmount","A lifecycle hook that calls a function when the component will unmount."],["use_update","A hook returns a function that forces component to re-render when called."],["use_websocket","This hook communicates with `WebSocket`."],["use_websocket_with_options","This hook communicates with `WebSocket` with options."],["use_window_scroll","A sensor hook that tracks Window scroll position."],["use_window_size","A sensor hook that tracks dimensions of the browser window."]],"mod":[["prelude",""]],"struct":[["CloseEvent","The `CloseEvent` class."],["LocationState","State for brower’s location."],["UseAsyncHandle","State handle for the [`use_async`] hook."],["UseAsyncOptions","Options for [`use_async_with_options`]."],["UseAsyncState","State for an async future."],["UseClipboardHandle","State handle for the [`use_clipboard`] hook."],["UseCounterHandle","State handle for the [`use_counter`] hook."],["UseDebounceHandle","State handle for the [`use_debounce`] hook."],["UseDebounceStateHandle","State handle for the [`use_debounce_state`] hook."],["UseDefaultHandle","State handle for the [`use_default`] hook."],["UseDragHandle","State handle for the [`use_drag`] hook."],["UseDragOptions","Options for drag."],["UseDropHandle","State handle for the [`use_drop`] hook."],["UseDropOptions","Options for drop."],["UseGeolocationOptions","The `PositionOptions` dictionary."],["UseGeolocationState",""],["UseHashHandle","State handle for the [`use_hash`] hook."],["UseLatestHandle","State handle for the [`use_latest`] hook."],["UseListHandle","State handle for the [`use_list`] hook."],["UseLocalStorageHandle","State handle for the [`use_local_storage`] hook."],["UseMapHandle","State handle for the [`use_map`] hook."],["UseMeasureState",""],["UseMediaHandle","State handle for the [`use_media`] hook."],["UseMediaOptions","Options for media"],["UseMutLatestHandle","State handle for the [`use_mut_latest`] hook."],["UsePreviousHandle","State handle for the [`use_previous`] hook."],["UseQueueHandle","State handle for the [`use_queue`] hook."],["UseRafStateHandle","State handle for the [`use_raf_state`] hook."],["UseSessionStorageHandle","State handle for the [`use_session_storage`] hook."],["UseSetHandle","State handle for the [`use_set`] hook."],["UseStatePtrEqHandle","State handle for the [`use_state_ptr_eq`] hook."],["UseSwipeHandle","State handle for the [`use_swipe`] hook."],["UseSwipeOptions","Options for swipe."],["UseThrottleHandle","State handle for the [`use_throttle`] hook."],["UseThrottleStateHandle","State handle for the [`use_throttle_state`] hook."],["UseTimeoutHandle","State handle for the [`use_timeout`] hook."],["UseToggleHandle","State handle for the [`use_toggle`] hook."],["UseWebSocketHandle","State handle for the [`use_websocket`] hook."],["UseWebSocketOptions","Options for `WebSocket`."]]};