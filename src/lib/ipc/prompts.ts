export async function prompt_respond(args: { id: string, value: string }): Promise<void> {
  console.log('Responded to prompt', args);
  return Promise.resolve();
}
