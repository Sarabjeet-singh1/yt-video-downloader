# Frontend Documentation

This directory contains the TypeScript React frontend for the YouTube Downloader application.

## Features

- **Modern React**: Built with React 18 and TypeScript
- **Responsive Design**: Mobile-first responsive layout with Tailwind CSS
- **Dark Mode Support**: Automatic dark/light theme switching
- **State Management**: Zustand for lightweight state management
- **Modern UI**: Glass morphism design with smooth animations

## Tech Stack

- **React 18**: Latest React with hooks and concurrent features
- **TypeScript**: Full type safety and better development experience
- **Vite**: Fast build tool and development server
- **Tailwind CSS**: Utility-first CSS framework
- **Zustand**: Lightweight state management
- **Lucide React**: Beautiful icons

## Development

### Prerequisites

- Node.js 16+ 
- npm or yarn

### Installation

```bash
npm install
```

### Development Server

```bash
npm run dev
```

Runs the app in development mode. Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

### Build

```bash
npm run build
```

Builds the app for production to the `dist` folder.

### Type Checking

```bash
npm run type-check
```

Runs TypeScript type checking without emitting files.

## Project Structure

```
src/
├── components/          # Reusable UI components
│   ├── layout/         # Layout components (Header, Sidebar)
│   └── ui/             # UI components (LoadingScreen, etc.)
├── pages/              # Page components
│   ├── HomePage.tsx    # Main dashboard page
│   ├── DownloadPage.tsx # Video download interface
│   └── SettingsPage.tsx # Settings configuration
├── hooks/              # Custom React hooks
│   └── useTheme.ts     # Theme management hook
├── store/              # State management
│   └── appStore.ts     # Main application store
├── types/              # TypeScript type definitions
│   └── index.ts        # Shared types
├── App.tsx             # Main application component
├── main.tsx            # Application entry point
└── index.css           # Global styles and Tailwind imports
```

## Key Components

### Layout Components

- **Header**: Top navigation bar with theme toggle and menu button
- **Sidebar**: Left navigation with feature highlights
- **LoadingScreen**: Splash screen with app branding

### Page Components

- **HomePage**: Dashboard with features overview and quick actions
- **DownloadPage**: Main download interface with URL input and options
- **SettingsPage**: Configuration options for download preferences

### State Management

The app uses Zustand for state management with the following stores:

- **App Store**: Global application state (loading, theme, etc.)
- **Theme Store**: Theme preferences and dark mode handling

## Styling

The app uses Tailwind CSS with custom components and utilities:

- **Design System**: Consistent color palette and spacing
- **Dark Mode**: Automatic dark/light theme switching
- **Glass Morphism**: Modern glass-like effects
- **Responsive**: Mobile-first responsive design
- **Animations**: Smooth transitions and micro-interactions

## API Integration

The frontend is designed to work with the Rust backend:

- **Download API**: Submit download requests with video URL and options
- **Progress Tracking**: Real-time download progress updates
- **Settings Sync**: Configuration synchronization with backend
- **Error Handling**: Comprehensive error handling and user feedback

## Deployment

The built files in the `dist` folder can be deployed to any static hosting service:

- Netlify
- Vercel  
- GitHub Pages
- AWS S3
- CloudFlare Pages

## Browser Support

- Chrome/Edge 88+
- Firefox 85+
- Safari 14+
- Mobile browsers (iOS Safari, Chrome Mobile)

## Contributing

1. Follow the existing code style and conventions
2. Use TypeScript for all new code
3. Write descriptive commit messages
4. Test changes in both light and dark themes
5. Ensure responsive design works on mobile devices

## Troubleshooting

### Common Issues

**Build Errors**
- Run `npm install` to ensure all dependencies are installed
- Clear cache with `rm -rf node_modules package-lock.json && npm install`

**TypeScript Errors**
- Run `npm run type-check` to see detailed type errors
- Ensure all imports use proper relative paths

**Theme Issues**
- Check that the `useTheme` hook is properly implemented
- Verify Tailwind dark mode classes are being applied correctly

