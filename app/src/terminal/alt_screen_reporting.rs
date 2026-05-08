use settings::{
    macros::define_settings_group, RespectUserSyncSetting, SupportedPlatforms, SyncToCloud,
};

define_settings_group!(AltScreenReporting, settings: [
    mouse_reporting_enabled: MouseReportingEnabled {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "terminal.mouse_reporting_enabled",
        description: "Whether to forward mouse events to full-screen terminal applications.",
    },
    scroll_reporting_enabled: ScrollReportingEnabled {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "terminal.scroll_reporting_enabled",
        description: "Whether to forward scroll events to full-screen terminal applications.",
    },
    focus_reporting_enabled: FocusReportingEnabled {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "terminal.focus_reporting_enabled",
        description: "Whether to forward focus and blur events to full-screen terminal applications.",
    },
    native_left_drag_select_enabled: NativeLeftDragSelectEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "terminal.native_left_drag_select_enabled",
        description: "When enabled, left mouse button events (click and drag) are handled by Warp's native selection instead of being forwarded to the running full-screen terminal application. This matches iTerm2's default behavior and lets Cmd+C copy the selection without holding Shift. Other mouse events (right-click, scroll wheel, modifier+click) still follow the regular mouse-reporting protocol.",
    },
]);
