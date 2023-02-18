import classNames from 'classnames';
import { useEffect } from 'react';
import styles from './Layout.module.css';

export function GradientBackground({ variant, className }) {
  const classes = classNames(
    {
      [styles.colorBackground]: variant === 'large',
      [styles.colorBackgroundBottom]: variant === 'small',
    },
    className
  );

  return <div className={classes} />;
}

export default function Layout({ children }) {
  const setAppTheme = () => {
    const darkMode = localStorage.getItem('theme') === 'dark';
    const lightMode = localStorage.getItem('theme') === 'light';

    if (darkMode) {
      document.documentElement.classList.add('dark');
    } else if (lightMode) {
      document.documentElement.classList.remove('dark');
    }
    return;
  };

  const handleSystemThemeChange = () => {
    var darkQuery = window.matchMedia('(prefers-color-scheme: dark)');

    darkQuery.onchange = (e) => {
      if (e.matches) {
        document.documentElement.classList.add('dark');
        localStorage.setItem('theme', 'dark');
      } else {
        document.documentElement.classList.remove('dark');
        localStorage.setItem('theme', 'light');
      }
    };
  };

  const dragAndDrop = () => {
    var dropArea = document.getElementById('drag-and-drop');

    // Prevent default drag behaviors
    ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
      dropArea.addEventListener(eventName, preventDefaults, false);
    });
    
    // Highlight drop area when dragging over
    ['dragenter', 'dragover'].forEach(eventName => {
      dropArea.addEventListener(eventName, highlight, false);
    });
    
    // Unhighlight drop area when dragging over
    ['dragleave', 'drop'].forEach(eventName => {
      dropArea.addEventListener(eventName, unhighlight, false);
    });
    
    // Handle dropped files
    dropArea.addEventListener('drop', handleDrop, false);
    
    function preventDefaults(e) {
      e.preventDefault();
      e.stopPropagation();
    }
    
    function highlight(e) {
      dropArea.classList.add('highlight');
    }
    
    function unhighlight(e) {
      dropArea.classList.remove('highlight');
    }
    
    function handleDrop(e) {
      var dt = e.dataTransfer;
      var files = dt.files;
    
      handleFiles(files);
    }
    
    function handleFiles(files) {
      files = [...files];
      files.forEach(uploadFile);
    }
    
    async function uploadFile(file) {
      const formData = new FormData();
      formData.append('file', file);

      try {
        console.log("uploadFile 2");
        const response = await fetch('/api/upload', {
          method: 'POST',
          body: formData,
        });
        console.log("uploadFile 3");
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        console.log("uploadFile 4");
        const { filename } = await response.json();
        console.log(response.json());
        console.log(`File uploaded successfully. Server returned filename: ${filename}`);
      } catch (error) {
        console.error('Error uploading file:', error);
      }
    }
  };

  useEffect(() => {
    dragAndDrop();
  }, []);

  useEffect(() => {
    setAppTheme();
  }, []);

  useEffect(() => {
    handleSystemThemeChange();
  }, []);

  return (
    <div className="relative pb-24 overflow-hidden">
    <div className="flex flex-col items-center max-w-2xl w-full mx-auto">
      {children}
      <div id="drag-and-drop" className="my-8 p-8 border-4 border-dashed border-gray-400 rounded-lg text-center">
        <p className="text-lg font-semibold text-gray-500">Drag and drop your image files here</p>
        <input type="file" id="file-input" accept=".jpg, .jpeg, .png" className="hidden" />
      </div>
    </div>
  </div>

  );
}
