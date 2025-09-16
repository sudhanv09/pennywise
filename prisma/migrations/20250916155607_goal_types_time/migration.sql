/*
  Warnings:

  - You are about to drop the column `deadline` on the `Goal` table. All the data in the column will be lost.
  - Added the required column `GoalType` to the `Goal` table without a default value. This is not possible if the table is not empty.
  - Added the required column `tillDate` to the `Goal` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Goal" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "title" TEXT NOT NULL,
    "GoalType" TEXT NOT NULL,
    "targetAmount" INTEGER NOT NULL,
    "currentAmount" INTEGER NOT NULL DEFAULT 0,
    "currentDate" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "tillDate" DATETIME NOT NULL,
    "userId" TEXT NOT NULL,
    CONSTRAINT "Goal_userId_fkey" FOREIGN KEY ("userId") REFERENCES "user" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Goal" ("currentAmount", "id", "targetAmount", "title", "userId") SELECT "currentAmount", "id", "targetAmount", "title", "userId" FROM "Goal";
DROP TABLE "Goal";
ALTER TABLE "new_Goal" RENAME TO "Goal";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
