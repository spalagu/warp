use settings::Setting as _;
use warpui::{AppContext, SingletonEntity};

use super::{
    alt_screen_reporting::AltScreenReporting, model::grid::grid_handler::TermMode, TerminalModel,
};

pub mod alt_screen_element;

/// Determines if mouse event is intercepted based on SGR_MOUSE mode and mouse reporting setting.
///
/// `is_left_button` indicates the event originates from the left mouse button (down, up, or drag).
/// When `native_left_drag_select_enabled` is on, left-button events are intercepted so that
/// drag-to-select uses Warp's native selection without requiring Shift (iTerm2-style behavior).
pub fn should_intercept_mouse(
    model: &TerminalModel,
    shift: bool,
    is_left_button: bool,
    ctx: &AppContext,
) -> bool {
    // Always intercept mouse for a shared session reader since their mouse events
    // will not be processed by the sharer's running terminal app.
    if model.shared_session_status().is_reader() || shift {
        return true;
    }
    let reporting = AltScreenReporting::as_ref(ctx);
    // When the iTerm2-style opt-in is enabled, intercept left-button events so the user can
    // drag-select natively in Warp without holding Shift. Other buttons (right, middle, scroll)
    // and modifier-clicks still follow the regular mouse-reporting path.
    if is_left_button && *reporting.native_left_drag_select_enabled.value() {
        return true;
    }
    // Require some level of mouse tracking to be enabled when the block list is active.
    let mouse_tracking = model.is_alt_screen_active()
        || model.is_term_mode_set(TermMode::MOUSE_REPORT_CLICK)
        || model.is_term_mode_set(TermMode::MOUSE_DRAG)
        || model.is_term_mode_set(TermMode::MOUSE_MOTION);
    let mouse_reporting_enabled = *reporting.mouse_reporting_enabled.value();
    !(model.is_term_mode_set(TermMode::SGR_MOUSE) && mouse_tracking && mouse_reporting_enabled)
}

/// Determines if scroll event is intercepted. SGR_mouse and mouse reporting must be enabled to
/// report scroll events, otherwise, always intercept scroll.
pub fn should_intercept_scroll(model: &TerminalModel, ctx: &AppContext) -> bool {
    let scroll_reporting_enabled = *AltScreenReporting::as_ref(ctx)
        .scroll_reporting_enabled
        .value();
    // Scroll is not a left-button event, so pass `is_left_button = false`.
    should_intercept_mouse(model, false, false, ctx) || !scroll_reporting_enabled
}
