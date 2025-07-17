/**
 * useTippy Hook - React Bit Pattern
 * 
 * A reusable hook that encapsulates Tippy.js logic following React Bits patterns.
 * This separates tooltip logic from component rendering logic.
 * 
 * React Bits Pattern: Custom Hook for Side Effects
 * Reference: https://reactbits.dev/
 */

import { useEffect, useRef, useCallback } from "react";
import tippy, { Instance, Props } from "tippy.js";
import "tippy.js/dist/tippy.css";

interface UseTippyOptions {
  content: string | (() => string);
  placement?: "top" | "bottom" | "left" | "right";
  delay?: [number, number];
  duration?: [number, number];
  maxWidth?: number;
  offset?: [number, number];
  theme?: string;
  dependencies?: any[];
}

export const useTippy = ({
  content,
  placement = "top",
  delay = [150, 0],
  duration = [200, 150],
  maxWidth = 320,
  offset = [0, 8],
  theme = "custom",
  dependencies = [],
}: UseTippyOptions) => {
  const elementRef = useRef<HTMLDivElement>(null);
  const tippyInstance = useRef<Instance<Props> | null>(null);

  // React Bit: Memoized content generator
  const getContent = useCallback(() => {
    return typeof content === "function" ? content() : content;
  }, [content]);

  // React Bit: Side effect management
  useEffect(() => {
    if (elementRef.current) {
      // Cleanup previous instance
      if (tippyInstance.current) {
        tippyInstance.current.destroy();
      }

      // Create new instance
      tippyInstance.current = tippy(elementRef.current, {
        content: getContent,
        allowHTML: true,
        placement,
        delay,
        duration,
        interactive: false,
        maxWidth,
        offset,
        zIndex: 9999,
        theme,
        arrow: false, // Disable arrow
      });

      // React Bit: Proper cleanup
      return () => {
        if (tippyInstance.current) {
          tippyInstance.current.destroy();
          tippyInstance.current = null;
        }
      };
    }
  }, [getContent, placement, delay, duration, maxWidth, offset, theme, ...dependencies]);

  return elementRef;
}; 