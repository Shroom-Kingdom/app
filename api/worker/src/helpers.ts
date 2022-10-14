export async function logErrorResponse(
  prefix: string,
  res: Response
): Promise<void> {
  console.error(`${prefix}: [${res.status}]`);
}
