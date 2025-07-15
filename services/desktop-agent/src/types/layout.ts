export interface ContentWindowProps {
  children?: React.ReactNode;
}

export interface SidebarProps {
  isCollapsed?: boolean;
  onToggle?: () => void;
} 