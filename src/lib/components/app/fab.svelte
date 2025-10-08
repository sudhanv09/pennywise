<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { cn } from "$lib/utils.js";
  import type { Component } from "svelte";

  type Position = "bottom-right" | "bottom-left";

  let {
    class: className,
    href = undefined,
    label = "Create",
    icon: Icon = null as Component | null,
    position = "bottom-right" as Position,
    disabled = false,
    children,
    ...restProps
  } = $props<{
    class?: string;
    href?: string;
    label?: string;
    icon?: Component | null;
    position?: Position;
    disabled?: boolean;
  }>();

  const positionClasses: Record<Position, string> = {
    "bottom-right": "bottom-6 right-6",
    "bottom-left": "bottom-6 left-6",
  };

  const resolvedLabel =
    typeof label === "string" && label.trim().length > 0 ? label : null;
</script>

<Button
  class={cn(
    "fixed z-40 h-14 w-14 rounded-full shadow-lg transition-all duration-200",
    "bg-primary text-primary-foreground hover:bg-primary/90 hover:-translate-y-0.5 hover:shadow-xl",
    "focus-visible:ring-[3px]",
    positionClasses[position],
    className
  )}
  {href}
  aria-label={resolvedLabel ?? undefined}
  {disabled}
  {...restProps}
>
  {#if children}
    {@render children?.()}
  {:else if Icon}
    <Icon class="h-6 w-6" />
    {#if resolvedLabel}
      <span class="sr-only">{resolvedLabel}</span>
    {/if}
  {:else if resolvedLabel}
    <span>{resolvedLabel}</span>
  {/if}
</Button>
