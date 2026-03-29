export type Action = () => void;

type ActionsMap = {
  closeOverlay: Action;
  navigateLeft: Action;
  navigateRight: Action;
  dismissDetail: Action;
  quickSwitch: (index: number) => void;
  refreshJira: Action;
};

let boundActions: ActionsMap | null = null;

export function registerBindings(actions: ActionsMap) {
  boundActions = actions;
}

export function handleKeydown(event: KeyboardEvent): void {
  if (!boundActions) return;

  if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
    return;
  }

  switch (event.key) {
    case 'Escape':
      boundActions.closeOverlay();
      break;
    case 'ArrowLeft':
      boundActions.navigateLeft();
      break;
    case 'ArrowRight':
      boundActions.navigateRight();
      break;
    case 'Backspace':
      boundActions.dismissDetail();
      break;
    case 'r':
    case 'R':
      boundActions.refreshJira();
      break;
    default:
      if (event.key >= '1' && event.key <= '9') {
        const index = parseInt(event.key, 10) - 1;
        boundActions.quickSwitch(index);
      }
      break;
  }
}
