import {
  createSignal,
  createContext,
  useContext,
  JSX,
  Component,
} from "solid-js";

type SidebarContextType = {
  isOpen: () => boolean;
  toggle: () => void;
  open: () => void;
  close: () => void;
  isMobile: () => boolean;
};

const SidebarContext = createContext<SidebarContextType>();

export function SidebarProvider(props: { children: JSX.Element }) {
  const [isOpen, setIsOpen] = createSignal(false);

  const toggle = () => setIsOpen(!isOpen());
  const open = () => setIsOpen(true);
  const close = () => setIsOpen(false);

  const isMobile = () => window.innerWidth < 768;

  return (
    <SidebarContext.Provider value={{ isOpen, toggle, open, close, isMobile }}>
      {props.children}
    </SidebarContext.Provider>
  );
}

export function useSidebar() {
  const context = useContext(SidebarContext);
  if (!context) {
    throw new Error("useSidebar must be used within a SidebarProvider");
  }
  return context;
}

interface SidebarTriggerProps {
  children: JSX.Element;
  "aria-label"?: string;
}

export const SidebarTrigger: Component<SidebarTriggerProps> = (props) => {
  const { toggle } = useSidebar();

  return (
    <button
      class="menu-toggle"
      onClick={toggle}
      aria-label={props["aria-label"] || "Toggle sidebar"}
    >
      {props.children}
    </button>
  );
};

interface Props {
  children: JSX.Element;
}

export const Sidebar: Component<Props> = (props) => {
  const { isOpen, close, isMobile } = useSidebar();

  return (
    <>
      <aside
        classList={{
          sidebar: true,
          open: isOpen(),
        }}
        aria-label="Sidebar"
      >
        {props.children}

        {/* Close button for mobile */}
        {isMobile() && (
          <button
            class="sidebar-close"
            onClick={close}
            aria-label="Close sidebar"
          >
            <i class="fas fa-times"></i>
          </button>
        )}
      </aside>

      {/* Overlay for mobile */}
      {isMobile() && (
        <div
          classList={{
            "sidebar-overlay": true,
            active: isOpen(),
          }}
          onClick={close}
          aria-hidden="true"
        />
      )}
    </>
  );
};

export const SidebarHeader: Component<Props> = (props) => {
  return (
    <div class="sidebar-header" role="banner">
      {props.children}
    </div>
  );
};

const SidebarTitle: Component<Props> = (props) => {
  return <h1 class="sidebar-title">{props.children}</h1>;
};

export const SidebarGroup: Component<Props> = (props) => {
  return (
    <nav class="sidebar-items" role="navigation">
      {props.children}
    </nav>
  );
};

interface SidebarItemProps {
  icon: JSX.Element;
  children: JSX.Element;
  href?: string;
  active?: boolean;
  "aria-current"?: "page" | "step" | "location" | "date" | "time" | true;
}

export const SidebarItem: Component<SidebarItemProps> = (props) => {
  return (
    <li class="sidebar-nav-item" role="none">
      <a
        href={props.href || "#"}
        classList={{
          "sidebar-nav-link": true,
          active: props.active,
        }}
        role="menuitem"
        aria-current={props["aria-current"]}
      >
        <span class="sidebar-nav-icon">{props.icon}</span>
        <span>{props.children}</span>
      </a>
    </li>
  );
};

export const SidebarFooter: Component<Props> = (props) => {
  return <footer class="sidebar-footer">{props.children}</footer>;
};
