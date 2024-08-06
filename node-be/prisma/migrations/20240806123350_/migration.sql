/*
  Warnings:

  - The `status` column on the `HabitLog` table would be dropped and recreated. This will lead to data loss if there is data in the column.
  - The `method` column on the `Reminder` table would be dropped and recreated. This will lead to data loss if there is data in the column.

*/
-- CreateEnum
CREATE TYPE "HabitStatus" AS ENUM ('Done', 'Missed', 'Pending');

-- CreateEnum
CREATE TYPE "ReminderMethod" AS ENUM ('Email', 'SMS', 'PushNotification');

-- AlterTable
ALTER TABLE "HabitLog" DROP COLUMN "status",
ADD COLUMN     "status" "HabitStatus" NOT NULL DEFAULT 'Pending';

-- AlterTable
ALTER TABLE "Reminder" DROP COLUMN "method",
ADD COLUMN     "method" "ReminderMethod" NOT NULL DEFAULT 'Email';
