import { createRoot } from 'react-dom/client';
import { App } from './ui/App.tsx';
import './ui/styles.css';

const floating = new URLSearchParams(window.location.search).get('view') === 'floating';
document.body.classList.toggle('floating-window', floating);
document.documentElement.classList.toggle('floating-window-root', floating);
createRoot(document.getElementById('root')!).render(<App floating={floating} />);
