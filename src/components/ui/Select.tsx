import { splitProps, Show, For, createMemo, type JSX } from 'solid-js';
import { Select as ArkSelect, createListCollection } from '@ark-ui/solid/select';
import { Field } from '@ark-ui/solid/field';
import { Portal } from 'solid-js/web';
import styles from './Select.module.css';

export interface SelectOption {
  value: string;
  label: string;
}

export interface SelectProps {
  label: string;
  options: SelectOption[];
  placeholder?: string;
  required?: boolean;
  value?: string;
  onChange?: JSX.EventHandler<HTMLInputElement, Event>;
  onBlur?: JSX.EventHandler<HTMLInputElement, FocusEvent>;
  error?: string;
  class?: string;
  id?: string;
  name?: string;
}

export function Select(props: SelectProps) {
  const [local, others] = splitProps(props, [
    'label',
    'options',
    'placeholder', 
    'required',
    'value',
    'onChange',
    'onBlur',
    'error',
    'class',
    'id',
    'name'
  ]);
  
  const selectId = () => local.id || `select-${Math.random().toString(36).substring(2, 11)}`;
  
  const collection = createMemo(() => 
    createListCollection({
      items: local.options,
    })
  );
  
  const className = () => {
    const baseClass = styles.field;
    const customClass = local.class || '';
    
    return [baseClass, customClass].filter(Boolean).join(' ');
  };

  const triggerClassName = () => {
    const baseClass = styles.trigger;
    const errorClass = local.error ? styles['trigger--error'] : '';
    
    return [baseClass, errorClass].filter(Boolean).join(' ');
  };

  return (
    <Field.Root class={className()}>
      <Field.Label class={styles.label} for={selectId()}>
        {local.label}
        <Show when={local.required}>
          <span class={styles.required}>*</span>
        </Show>
      </Field.Label>
      
      <ArkSelect.Root
        collection={collection()}
        value={local.value ? [local.value] : []}
        onValueChange={(details) => local.onChange?.(details.value[0])}
        {...others}
      >
        <ArkSelect.Control class={styles.control}>
          <ArkSelect.Trigger id={selectId()} class={triggerClassName()}>
            <ArkSelect.ValueText placeholder={local.placeholder || 'Select an option'} />
            <ArkSelect.Indicator class={styles.indicator}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="6,9 12,15 18,9"></polyline>
              </svg>
            </ArkSelect.Indicator>
          </ArkSelect.Trigger>
        </ArkSelect.Control>
        
        <Portal>
          <ArkSelect.Positioner>
            <ArkSelect.Content class={styles.content}>
              <For each={collection().items}>
                {(item) => (
                  <ArkSelect.Item class={styles.item} item={item}>
                    <ArkSelect.ItemText>{item.label}</ArkSelect.ItemText>
                    <ArkSelect.ItemIndicator class={styles.itemIndicator}>
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="20,6 9,17 4,12"></polyline>
                      </svg>
                    </ArkSelect.ItemIndicator>
                  </ArkSelect.Item>
                )}
              </For>
            </ArkSelect.Content>
          </ArkSelect.Positioner>
        </Portal>
        
        <ArkSelect.HiddenSelect />
      </ArkSelect.Root>
      
      <Show when={local.error}>
        <Field.ErrorText class={styles.error}>
          {local.error}
        </Field.ErrorText>
      </Show>
    </Field.Root>
  );
}