/**
 * Tauri Command Contracts - Frontend/Backend Interface
 * 
 * These TypeScript interfaces define the contract between the 
 * Vue.js frontend and Rust Tauri backend commands.
 */

// ============================================================================
// SLEEP PREVENTION COMMANDS
// ============================================================================

/**
 * Start sleep prevention for specified duration
 */
export interface StartPreventionCommand {
  hours: number;        // 0-8 hours
  minutes: number;      // 0-59 minutes  
  seconds: number;      // 0-59 seconds
}

export interface StartPreventionResponse {
  success: boolean;
  sessionId: string;    // Unique session identifier
  message?: string;     // Success/error message
  error?: string;       // Error details if success=false
}

/**
 * Stop active sleep prevention
 */
export interface StopPreventionResponse {
  success: boolean;
  message?: string;
  wasActive: boolean;   // Whether prevention was actually running
}

/**
 * Get current sleep prevention status
 */
export interface GetStatusResponse {
  isActive: boolean;
  sessionId?: string;
  startTime?: string;   // ISO date string
  totalDuration: number;     // Total seconds
  remainingTime: number;     // Remaining seconds
  status: 'inactive' | 'active' | 'expired' | 'cancelled' | 'error';
  error?: string;
}

// ============================================================================
// TRAY MANAGEMENT COMMANDS
// ============================================================================

/**
 * Update system tray icon and tooltip
 */
export interface UpdateTrayCommand {
  iconType: 'moon-hollow' | 'sun-filled' | 'check-mark';
  tooltipText: string;
}

export interface UpdateTrayResponse {
  success: boolean;
  error?: string;
}

/**
 * Update tray menu items
 */
export interface TrayMenuItem {
  id: string;
  label: string;
  enabled: boolean;
  separator?: boolean;
}

export interface UpdateTrayMenuCommand {
  items: TrayMenuItem[];
}

export interface UpdateTrayMenuResponse {
  success: boolean;
  error?: string;
}

// ============================================================================
// CONFIGURATION COMMANDS
// ============================================================================

/**
 * Get app configuration
 */
export interface GetConfigResponse {
  maxDurationHours: number;
  defaultDuration: {
    hours: number;
    minutes: number;
    seconds: number;
  };
  showNotifications: boolean;
  autoStart: boolean;
  theme: 'auto' | 'light' | 'dark';
}

/**
 * Update app configuration
 */
export interface UpdateConfigCommand {
  maxDurationHours?: number;
  defaultDuration?: {
    hours: number;
    minutes: number;
    seconds: number;
  };
  showNotifications?: boolean;
  autoStart?: boolean;
  theme?: 'auto' | 'light' | 'dark';
}

export interface UpdateConfigResponse {
  success: boolean;
  error?: string;
}

// ============================================================================
// SYSTEM INFO COMMANDS
// ============================================================================

/**
 * Get system information for compatibility checking
 */
export interface GetSystemInfoResponse {
  macosVersion: string;     // e.g., "13.4.1"
  supportsIOKit: boolean;   // Whether IOKit APIs are available
  hasPermissions: boolean;  // Whether app has required permissions
  batteryLevel?: number;    // 0-100 percentage if available
  isOnBattery: boolean;    // Whether running on battery power
}

/**
 * Validate time duration input
 */
export interface ValidateDurationCommand {
  hours: number;
  minutes: number;
  seconds: number;
}

export interface ValidateDurationResponse {
  isValid: boolean;
  totalSeconds: number;
  errors: string[];        // List of validation error messages
  warnings: string[];      // List of warning messages (e.g., battery drain)
}

// ============================================================================
// EVENT TYPES (Frontend → Backend)
// ============================================================================

/**
 * Events that the frontend can listen to from the backend
 */
export type TauriEvent = 
  | 'session-started'
  | 'session-ended' 
  | 'session-cancelled'
  | 'time-remaining'
  | 'system-error'
  | 'user-session-changed'
  | 'battery-warning';

export interface SessionStartedEvent {
  sessionId: string;
  duration: number;
  startTime: string;
}

export interface SessionEndedEvent {
  sessionId: string;
  reason: 'expired' | 'cancelled' | 'error' | 'user-logout';
}

export interface TimeRemainingEvent {
  sessionId: string;
  remainingSeconds: number;
  percentComplete: number;
}

export interface SystemErrorEvent {
  error: string;
  recoverable: boolean;
  sessionId?: string;
}

export interface BatteryWarningEvent {
  batteryLevel: number;
  recommendCancel: boolean;
  message: string;
}

// ============================================================================
// TAURI INVOKE FUNCTION NAMES
// ============================================================================

/**
 * Tauri command names for invoke() calls
 */
export const TAURI_COMMANDS = {
  // Sleep Prevention
  START_PREVENTION: 'start_sleep_prevention',
  STOP_PREVENTION: 'stop_sleep_prevention', 
  GET_STATUS: 'get_sleep_status',
  
  // Tray Management
  UPDATE_TRAY_ICON: 'update_tray_icon',
  UPDATE_TRAY_MENU: 'update_tray_menu',
  
  // Configuration
  GET_CONFIG: 'get_app_config',
  UPDATE_CONFIG: 'update_app_config',
  
  // System Info
  GET_SYSTEM_INFO: 'get_system_info',
  VALIDATE_DURATION: 'validate_duration',
} as const;