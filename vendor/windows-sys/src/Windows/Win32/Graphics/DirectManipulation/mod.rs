pub type IDirectManipulationAutoScrollBehavior = *mut ::core::ffi::c_void;
pub type IDirectManipulationCompositor = *mut ::core::ffi::c_void;
pub type IDirectManipulationCompositor2 = *mut ::core::ffi::c_void;
pub type IDirectManipulationContent = *mut ::core::ffi::c_void;
pub type IDirectManipulationDeferContactService = *mut ::core::ffi::c_void;
pub type IDirectManipulationDragDropBehavior = *mut ::core::ffi::c_void;
pub type IDirectManipulationDragDropEventHandler = *mut ::core::ffi::c_void;
pub type IDirectManipulationFrameInfoProvider = *mut ::core::ffi::c_void;
pub type IDirectManipulationInteractionEventHandler = *mut ::core::ffi::c_void;
pub type IDirectManipulationManager = *mut ::core::ffi::c_void;
pub type IDirectManipulationManager2 = *mut ::core::ffi::c_void;
pub type IDirectManipulationManager3 = *mut ::core::ffi::c_void;
pub type IDirectManipulationPrimaryContent = *mut ::core::ffi::c_void;
pub type IDirectManipulationUpdateHandler = *mut ::core::ffi::c_void;
pub type IDirectManipulationUpdateManager = *mut ::core::ffi::c_void;
pub type IDirectManipulationViewport = *mut ::core::ffi::c_void;
pub type IDirectManipulationViewport2 = *mut ::core::ffi::c_void;
pub type IDirectManipulationViewportEventHandler = *mut ::core::ffi::c_void;
pub const CLSID_AutoScrollBehavior: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 638741073, data2: 15472, data3: 19610, data4: [174, 194, 148, 136, 73, 238, 176, 147] };
pub const CLSID_DeferContactService: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3619060980, data2: 33979, data3: 17230, data4: [134, 174, 101, 146, 187, 201, 171, 217] };
pub const CLSID_DragDropConfigurationBehavior: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 162536254, data2: 47724, data3: 17741, data4: [130, 232, 149, 227, 82, 50, 159, 35] };
pub const CLSID_HorizontalIndicatorContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3889270005, data2: 16071, data3: 17621, data4: [167, 107, 55, 112, 243, 207, 144, 61] };
pub const CLSID_VerticalIndicatorContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2701877015, data2: 45024, data3: 19106, data4: [145, 233, 62, 112, 1, 210, 230, 180] };
pub const CLSID_VirtualViewportContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 839295386, data2: 34544, data3: 19636, data4: [167, 243, 22, 227, 183, 226, 216, 82] };
pub const DCompManipulationCompositor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2044634663, data2: 41098, data3: 17324, data4: [142, 245, 105, 0, 185, 41, 145, 38] };
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
pub const DirectManipulationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1424101814, data2: 13904, data3: 20341, data4: [131, 52, 250, 53, 149, 152, 225, 197] };
pub const DirectManipulationPrimaryContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3399493217, data2: 54686, data3: 16839, data4: [131, 147, 59, 163, 186, 203, 107, 87] };
pub const DirectManipulationSharedManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2574856838, data2: 30668, data3: 19287, data4: [150, 219, 59, 53, 79, 111, 159, 181] };
pub const DirectManipulationUpdateManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2680274901, data2: 6197, data3: 17434, data4: [179, 177, 182, 204, 116, 183, 39, 208] };
pub const DirectManipulationViewport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 887230902, data2: 13904, data3: 20341, data4: [131, 52, 250, 53, 149, 152, 225, 197] };
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_DRAG_DROP_STATUS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_GESTURE_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_HITTEST_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_HORIZONTALALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_INPUT_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_INTERACTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = 100i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_MOTION_TYPES = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = 55i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_SNAPPOINT_COORDINATE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_SNAPPOINT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_STATUS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_VERTICALALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_VIEWPORT_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 16i32;
