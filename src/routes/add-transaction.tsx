import { createFileRoute, useNavigate } from '@tanstack/solid-router';
import { createSignal } from 'solid-js';
import { TransactionForm } from '../components/forms/TransactionForm';
import type { TransactionData } from '../components/forms/TransactionForm';
import './add-transaction.css';

export const Route = createFileRoute('/add-transaction')({
  component: AddTransactionComponent,
});

function AddTransactionComponent() {
  const navigate = useNavigate();
  const [isSubmitting, setIsSubmitting] = createSignal(false);
  const [showSuccess, setShowSuccess] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  const handleSubmit = async (transactionData: TransactionData) => {
    setIsSubmitting(true);
    setError(null);
    
    try {
      console.log('Submitting transaction:', transactionData);
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      setShowSuccess(true);
      
      setTimeout(() => {
        setShowSuccess(false);
        navigate({ to: '/dashboard' });
      }, 2000);
      
    } catch (err) {
      setError('Failed to save transaction. Please try again.');
      console.error('Error saving transaction:', err);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleCancel = () => {
    navigate({ to: '/dashboard' });
  };

  return (
    <div class="addTransaction">
      <div class="addTransaction__container">
        <div class="addTransaction__header">
          <h1 class="addTransaction__title">Add New Transaction</h1>
          <p class="addTransaction__subtitle">
            Record a new expense or income transaction
          </p>
        </div>

        {showSuccess() && (
          <div class="addTransaction__success">
            <div class="addTransaction__successIcon">✓</div>
            <div class="addTransaction__successMessage">
              <h3>Transaction Added Successfully!</h3>
              <p>Redirecting to dashboard...</p>
            </div>
          </div>
        )}

        {error() && (
          <div class="addTransaction__error">
            <div class="addTransaction__errorIcon">⚠</div>
            <div class="addTransaction__errorMessage">
              {error()}
            </div>
            <button 
              class="addTransaction__errorClose"
              onClick={() => setError(null)}
            >
              ×
            </button>
          </div>
        )}

        <div class="addTransaction__formContainer">
          <TransactionForm
            onSubmit={handleSubmit}
            mode="create"
            class="addTransaction__form"
          />
          
          <div class="addTransaction__actions">
            <button
              type="button"
              class="addTransaction__cancelButton"
              onClick={handleCancel}
              disabled={isSubmitting()}
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}