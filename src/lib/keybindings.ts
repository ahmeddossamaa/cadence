export type Action = () => void;

type ActionsMap = {
  toggleOverlay: Action;
  closeOverlay: Action;
  navigateLeft: Action;
  navigateRight: Action;
  selectTicket: Action;
  scrollUp: Action;
  scrollDown: Action;
  cycleFocus: Action;
  quickSwitch: (index: number) => void;
  togglePause: Action;
  refreshJira: Action;
  openSettings: Action;
  acceptPrompt: Action;
  dismissPrompt: Action;
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
    case 'Enter':
      boundActions.selectTicket();
      break;
    case 'ArrowUp':
      boundActions.scrollUp();
      break;
    case 'ArrowDown':
      boundActions.scrollDown();
      break;
    case 'Tab':
      event.preventDefault();
      boundActions.cycleFocus();
      break;
    case ' ':
      event.preventDefault();
      boundActions.togglePause();
      break;
    case 'r':
    case 'R':
      boundActions.refreshJira();
      break;
    case 's':
    case 'S':
      boundActions.openSettings();
      break;
    case 'y':
    case 'Y':
      boundActions.acceptPrompt();
      break;
    case 'n':
    case 'N':
      boundActions.dismissPrompt();
      break;
    default:
      if (event.key >= '1' && event.key <= '9') {
        const index = parseInt(event.key, 10) - 1;
        boundActions.quickSwitch(index);
      }
      break;
  }
}
