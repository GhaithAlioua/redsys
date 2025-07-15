export interface LayoutProps {
  children: React.ReactNode;
}

export interface SidebarProps {
  isCollapsed?: boolean;
  onToggle?: () => void;
}

export interface StatusBarProps {
  status?: string;
  progress?: number;
}

export interface ContentWindowProps {
  children?: React.ReactNode;
} 