import { createFileRoute } from '@tanstack/solid-router';
import { createSignal, For, Show } from 'solid-js';
import { Badge } from '../components/ui/Badge';
import { Button } from '../components/ui/Button';
import { Input } from '../components/ui/Input';
import { Select } from '../components/ui/Select';
import type { BadgeVariant } from '../components/ui/Badge';
import './transactions.css';

export const Route = createFileRoute('/transactions')({
  component: TransactionsComponent,
});

// Mock transaction data - in real app this would come from API/store
interface Transaction {
  id: string;
  title: string;
  amount: number;
  date: string;
  category: 'shopping' | 'bills' | 'groceries' | 'entertainment' | 'transport' | 'other';
  paymentMethod: 'cash' | 'card';
  type: 'normal' | 'loan' | 'goal';
  description?: string;
}

function TransactionsComponent() {
  // Mock data
  const [transactions, setTransactions] = createSignal<Transaction[]>([
    {
      id: '1',
      title: 'Grocery Shopping',
      amount: 85.50,
      date: '2024-01-15',
      category: 'groceries',
      paymentMethod: 'card',
      type: 'normal',
      description: 'Weekly grocery shopping at Whole Foods'
    },
    {
      id: '2',
      title: 'Electric Bill',
      amount: 120.00,
      date: '2024-01-14',
      category: 'bills',
      paymentMethod: 'card',
      type: 'normal'
    },
    {
      id: '3',
      title: 'Coffee Shop',
      amount: 4.75,
      date: '2024-01-14',
      category: 'entertainment',
      paymentMethod: 'cash',
      type: 'normal',
      description: 'Morning coffee'
    },
    {
      id: '4',
      title: 'Gas Station',
      amount: 45.20,
      date: '2024-01-13',
      category: 'transport',
      paymentMethod: 'card',
      type: 'normal'
    },
    {
      id: '5',
      title: 'Online Shopping',
      amount: 156.99,
      date: '2024-01-12',
      category: 'shopping',
      paymentMethod: 'card',
      type: 'normal',
      description: 'New headphones and accessories'
    },
    {
      id: '6',
      title: 'Loan Payment',
      amount: 300.00,
      date: '2024-01-10',
      category: 'other',
      paymentMethod: 'card',
      type: 'loan',
      description: 'Monthly car loan payment'
    }
  ]);

  // Filter and search state
  const [searchTerm, setSearchTerm] = createSignal('');
  const [categoryFilter, setCategoryFilter] = createSignal('all');
  const [typeFilter, setTypeFilter] = createSignal('all');

  // Filter options
  const categoryOptions = [
    { value: 'all', label: 'All Categories' },
    { value: 'shopping', label: 'Shopping' },
    { value: 'bills', label: 'Bills' },
    { value: 'groceries', label: 'Groceries' },
    { value: 'entertainment', label: 'Entertainment' },
    { value: 'transport', label: 'Transport' },
    { value: 'other', label: 'Other' }
  ];

  const typeOptions = [
    { value: 'all', label: 'All Types' },
    { value: 'normal', label: 'Normal' },
    { value: 'loan', label: 'Loan' },
    { value: 'goal', label: 'Goal' }
  ];

  // Filtered transactions
  const filteredTransactions = () => {
    return transactions().filter(transaction => {
      const matchesSearch = transaction.title.toLowerCase().includes(searchTerm().toLowerCase()) ||
                           (transaction.description?.toLowerCase().includes(searchTerm().toLowerCase()) ?? false);
      const matchesCategory = categoryFilter() === 'all' || transaction.category === categoryFilter();
      const matchesType = typeFilter() === 'all' || transaction.type === typeFilter();
      
      return matchesSearch && matchesCategory && matchesType;
    });
  };

  // Format currency
  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 2,
    }).format(amount);
  };

  // Format date
  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  };

  // Get payment method icon
  const getPaymentIcon = (method: string) => {
    return method === 'cash' ? '💵' : '💳';
  };

  // Clear filters
  const clearFilters = () => {
    setSearchTerm('');
    setCategoryFilter('all');
    setTypeFilter('all');
  };

  return (
    <div class="transactions">
      <div class="transactions__header">
        <h1 class="transactions__title">Transaction History</h1>
        <p class="transactions__subtitle">
          View and manage all your financial transactions
        </p>
      </div>

      {/* Filters */}
      <div class="transactions__filters">
        <div class="transactions__filterGroup">
          <Input
            label="Search transactions"
            placeholder="Search by title or description..."
            value={searchTerm()}
            onInput={setSearchTerm}
            class="transactions__searchInput"
          />
        </div>
        
        <div class="transactions__filterGroup">
          <Select
            label="Category"
            options={categoryOptions}
            value={categoryFilter()}
            onChange={setCategoryFilter}
            class="transactions__filterSelect"
          />
        </div>
        
        <div class="transactions__filterGroup">
          <Select
            label="Type"
            options={typeOptions}
            value={typeFilter()}
            onChange={setTypeFilter}
            class="transactions__filterSelect"
          />
        </div>
        
        <div class="transactions__filterActions">
          <Button
            variant="outline"
            onClick={clearFilters}
            class="transactions__clearButton"
          >
            Clear Filters
          </Button>
        </div>
      </div>

      {/* Results count */}
      <div class="transactions__results">
        <p class="transactions__count">
          Showing {filteredTransactions().length} of {transactions().length} transactions
        </p>
      </div>

      {/* Desktop Table View */}
      <div class="transactions__tableContainer">
        <table class="transactions__table">
          <thead class="transactions__tableHeader">
            <tr>
              <th>Date</th>
              <th>Title</th>
              <th>Category</th>
              <th>Amount</th>
              <th>Payment</th>
              <th>Type</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody class="transactions__tableBody">
            <For each={filteredTransactions()}>
              {(transaction) => (
                <tr class="transactions__tableRow">
                  <td class="transactions__tableCell transactions__tableCell--date">
                    {formatDate(transaction.date)}
                  </td>
                  <td class="transactions__tableCell transactions__tableCell--title">
                    {transaction.title}
                  </td>
                  <td class="transactions__tableCell transactions__tableCell--category">
                    <Badge variant={transaction.category as BadgeVariant}>
                      {transaction.category}
                    </Badge>
                  </td>
                  <td class="transactions__tableCell transactions__tableCell--amount">
                    {formatCurrency(transaction.amount)}
                  </td>
                  <td class="transactions__tableCell transactions__tableCell--payment">
                    <span class="transactions__paymentMethod">
                      {getPaymentIcon(transaction.paymentMethod)} {transaction.paymentMethod}
                    </span>
                  </td>
                  <td class="transactions__tableCell transactions__tableCell--type">
                    <span class={`transactions__type transactions__type--${transaction.type}`}>
                      {transaction.type}
                    </span>
                  </td>
                  <td class="transactions__tableCell transactions__tableCell--description">
                    {transaction.description || '—'}
                  </td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </div>

      {/* Mobile Card View */}
      <div class="transactions__cardContainer">
        <For each={filteredTransactions()}>
          {(transaction) => (
            <div class="transactions__card">
              <div class="transactions__cardHeader">
                <div class="transactions__cardTitle">
                  <h3>{transaction.title}</h3>
                  <span class="transactions__cardDate">
                    {formatDate(transaction.date)}
                  </span>
                </div>
                <div class="transactions__cardAmount">
                  {formatCurrency(transaction.amount)}
                </div>
              </div>
              
              <div class="transactions__cardBody">
                <div class="transactions__cardMeta">
                  <Badge variant={transaction.category as BadgeVariant}>
                    {transaction.category}
                  </Badge>
                  
                  <span class="transactions__cardPayment">
                    {getPaymentIcon(transaction.paymentMethod)} {transaction.paymentMethod}
                  </span>
                  
                  <span class={`transactions__cardType transactions__type--${transaction.type}`}>
                    {transaction.type}
                  </span>
                </div>
                
                <Show when={transaction.description}>
                  <p class="transactions__cardDescription">
                    {transaction.description}
                  </p>
                </Show>
              </div>
            </div>
          )}
        </For>
      </div>

      {/* Empty state */}
      <Show when={filteredTransactions().length === 0}>
        <div class="transactions__empty">
          <div class="transactions__emptyIcon">📊</div>
          <h3 class="transactions__emptyTitle">No transactions found</h3>
          <p class="transactions__emptyMessage">
            {transactions().length === 0 
              ? "You haven't recorded any transactions yet."
              : "Try adjusting your search or filter criteria."
            }
          </p>
          <Show when={searchTerm() || categoryFilter() !== 'all' || typeFilter() !== 'all'}>
            <Button variant="primary" onClick={clearFilters}>
              Clear Filters
            </Button>
          </Show>
        </div>
      </Show>
    </div>
  );
}