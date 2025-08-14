CREATE TABLE `goals` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`target_amount` real NOT NULL,
	`current_amount` real DEFAULT 0 NOT NULL,
	`target_date` text,
	`description` text,
	`created_at` text DEFAULT (CURRENT_TIMESTAMP),
	`is_completed` integer DEFAULT false NOT NULL
);
--> statement-breakpoint
CREATE TABLE `loans` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`principal_amount` real NOT NULL,
	`remaining_amount` real NOT NULL,
	`interest_rate` real NOT NULL,
	`monthly_payment` real NOT NULL,
	`start_date` text NOT NULL,
	`end_date` text,
	`lender_name` text,
	`is_active` integer DEFAULT true NOT NULL,
	`created_at` text DEFAULT (CURRENT_TIMESTAMP)
);
--> statement-breakpoint
CREATE TABLE `subscriptions` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`amount` real NOT NULL,
	`frequency` text NOT NULL,
	`next_payment_date` text NOT NULL,
	`category_id` integer NOT NULL,
	`account_id` text NOT NULL,
	`is_active` integer DEFAULT true NOT NULL,
	`created_at` text DEFAULT (CURRENT_TIMESTAMP),
	FOREIGN KEY (`category_id`) REFERENCES `categories`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`account_id`) REFERENCES `accounts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `user_settings` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`default_currency` text(3) DEFAULT 'USD' NOT NULL,
	`date_format` text DEFAULT 'MM/DD/YYYY' NOT NULL,
	`theme` text DEFAULT 'system' NOT NULL,
	`updated_at` text DEFAULT (CURRENT_TIMESTAMP)
);
--> statement-breakpoint
ALTER TABLE `transactions` ADD `category_id` integer REFERENCES categories(id);--> statement-breakpoint
ALTER TABLE `transactions` ADD `goal_id` text REFERENCES goals(id);--> statement-breakpoint
ALTER TABLE `transactions` ADD `subscription_id` text REFERENCES subscriptions(id);--> statement-breakpoint
ALTER TABLE `transactions` ADD `loan_id` text REFERENCES loans(id);