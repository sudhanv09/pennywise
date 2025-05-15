import Dialog from "@corvu/dialog";
import { VoidComponent } from "solid-js";
import './transaction-modal.module.css'

export default function TransactionModal<VoidComponent>() {
  return (
    <Dialog>
      <Dialog.Trigger>Add Transaction</Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content>
          <Dialog.Label>Add a new transaction</Dialog.Label>
          Hello
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog>
  );
}
