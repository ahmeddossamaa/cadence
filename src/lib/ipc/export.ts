export async function export_csv(args: { path?: string }): Promise<string> {
  return Promise.resolve(args.path || '/tmp/export.csv');
}
