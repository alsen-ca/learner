import ctypes
import platform

# Cross-platform screen size fetching
def get_screen_size():
    if platform.system() == "Windows":
        user32 = ctypes.windll.user32
        user32.SetProcessDPIAware()
        return user32.GetSystemMetrics(0), user32.GetSystemMetrics(1) # width, height
    else:
        # Linux fallback
        import tkinter as tk
        root = tk.Tk()
        root.withdraw()
        width = root.winfo_screenwidth()
        height = root.winfo_screenheight()
        root.destroy()
        return width, height