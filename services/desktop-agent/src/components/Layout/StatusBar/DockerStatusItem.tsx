/**
 * Professional Docker Status Item Component
 *
 * This component provides a polished Docker status display with professional styling
 * and a beautiful tooltip that matches the desktop agent's design system.
 */

import React, { useMemo, useCallback } from "react";
import Tippy from "@tippyjs/react";
import { AlertCircle, CheckCircle, Clock } from "lucide-react";
import useDocker from "../../../hooks/useDocker";

const cn = (...classes: (string | undefined | null | false)[]): string => {
  return classes.filter(Boolean).join(" ");
};

export const DockerStatusItem: React.FC = React.memo(() => {
  const { data: dockerInfo, isLoading, error, refresh } = useDocker();

  // Memoize status information to prevent recalculation on every render
  const statusInfo = useMemo(() => {
    if (isLoading) {
      return {
        iconColor: "text-gray-400",
        icon: Clock,
        status: "Checking...",
        statusColor: "text-gray-400",
        bgColor: "bg-gray-500",
        borderColor: "border-gray-600",
      };
    }

    if (error || !dockerInfo || !dockerInfo.is_available) {
      return {
        iconColor: "text-red-400",
        icon: AlertCircle,
        status: "Not Running",
        statusColor: "text-red-400",
        bgColor: "bg-red-500",
        borderColor: "border-red-600",
      };
    }

    return {
      iconColor: "text-green-400",
      icon: CheckCircle,
      status: "Running",
      statusColor: "text-green-400",
      bgColor: "bg-green-500",
      borderColor: "border-green-600",
    };
  }, [isLoading, error, dockerInfo]);

  const IconComponent = statusInfo.icon;

  // Memoize tooltip content to prevent recreation on every render
  const tooltipContent = useMemo(
    () => (
      <div className="bg-dark-editor border border-dark-border rounded-lg shadow-xl p-4 min-w-[280px]">
        {/* Header */}
        <div className="flex items-center gap-3 mb-4 pb-3 border-b border-dark-border">
          <div
            className={cn(
              "w-2.5 h-2.5 rounded-full border",
              statusInfo.bgColor,
              statusInfo.borderColor
            )}
          />
          <div>
            <h3 className="text-dark-text font-semibold text-sm">
              Docker Engine
            </h3>
            <p className={cn("text-xs font-medium", statusInfo.statusColor)}>
              {statusInfo.status}
            </p>
          </div>
        </div>

        {/* Status Details */}
        <div className="space-y-3">
          {/* Engine Status */}
          <div className="flex items-center justify-between">
            <span className="text-dark-textMuted text-xs font-medium">
              Status
            </span>
            <span
              className={cn("text-xs font-semibold", statusInfo.statusColor)}
            >
              {statusInfo.status}
            </span>
          </div>

          {/* Engine Version */}
          {dockerInfo?.is_available && dockerInfo.version && (
            <div className="flex items-center justify-between">
              <span className="text-dark-textMuted text-xs font-medium">
                Engine
              </span>
              <span className="text-dark-text font-mono text-xs bg-dark-highlight px-2 py-1 rounded">
                {dockerInfo.version}
              </span>
            </div>
          )}

          {/* Last Check */}
          <div className="flex items-center justify-between">
            <span className="text-dark-textMuted text-xs font-medium">
              Last Check
            </span>
            <span className="text-dark-text text-xs">
              {dockerInfo?.last_health_check
                ? new Date(dockerInfo.last_health_check).toLocaleTimeString()
                : "Never"}
            </span>
          </div>

          {/* Error Message (if any) */}
          {dockerInfo?.connection_error && (
            <div className="pt-2 border-t border-dark-border">
              <span className="text-dark-textMuted text-xs font-medium block mb-1">
                Error
              </span>
              <span className="text-red-400 text-xs leading-relaxed">
                {dockerInfo.connection_error}
              </span>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="mt-4 pt-3 border-t border-dark-border">
          <p className="text-dark-textMuted text-xs">Click to refresh status</p>
        </div>
      </div>
    ),
    [statusInfo, dockerInfo]
  );

  // Memoize click handler to prevent recreation
  const handleClick = useCallback(() => {
    if (!isLoading) {
      refresh();
    }
  }, [isLoading, refresh]);

  return (
    <Tippy
      content={tooltipContent}
      placement="top"
      arrow={true}
      delay={[300, 0]}
      duration={[200, 150]}
      className="tippy-custom"
      theme="custom"
    >
      <button
        onClick={handleClick}
        disabled={isLoading}
        className={cn(
          // Base styles
          "flex items-center gap-2 px-3 py-1.5 transition-all duration-200",
          "hover:bg-dark-highlight focus:outline-none focus:ring-2 focus:ring-blue-500/30",
          "border border-transparent hover:border-dark-border",
          "group relative",

          // Loading state
          isLoading && "cursor-not-allowed opacity-70"
        )}
      >
        {/* Status Circle Icon */}
        <div className="relative">
          <div
            className={cn(
              "w-2 h-2 rounded-full border transition-all duration-200",
              statusInfo.bgColor,
              statusInfo.borderColor,
              "group-hover:scale-110"
            )}
          />
          <IconComponent
            size={8}
            className={cn(
              "absolute inset-0 m-auto transition-all duration-200",
              statusInfo.iconColor,
              "group-hover:scale-110"
            )}
          />
        </div>

        {/* Docker Label */}
        <span className="text-dark-text text-xs font-medium tracking-wide">
          Docker
        </span>

        {/* Loading indicator */}
        {isLoading && (
          <div className="w-1.5 h-1.5 rounded-full bg-gray-400 animate-pulse" />
        )}
      </button>
    </Tippy>
  );
});

DockerStatusItem.displayName = "DockerStatusItem";

export default DockerStatusItem;
