//connection config with database
import { PrismaClient } from "@prisma/client";
const prisma = new PrismaClient

export default prisma
export { prisma }