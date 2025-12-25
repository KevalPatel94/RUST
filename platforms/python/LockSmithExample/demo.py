#!/usr/bin/env python3
"""
LockSmith Demo App

Interactive console-based demo app showing password validation with localization.
Similar to the Android/iOS example apps but running in the terminal.
"""

import sys
import argparse
from typing import Optional
from getpass import getpass

# Import the locksmith bindings
try:
    from . import locksmith
except ImportError:
    # Allow running as a script
    import locksmith


class LocaleOption:
    """Available locale options"""
    ENGLISH = ("en", "English")
    SPANISH = ("es", "Español")
    FRENCH = ("fr", "Français")
    
    @classmethod
    def all(cls):
        return [cls.ENGLISH, cls.SPANISH, cls.FRENCH]
    
    @classmethod
    def from_code(cls, code: str):
        for locale_code, name in cls.all():
            if locale_code == code:
                return locale_code, name
        return cls.ENGLISH


class SamplePassword:
    """Sample passwords for testing validation"""
    SAMPLES = [
        ("password-sample-too-short", "Ab1!"),
        ("password-sample-too-long", "Abcdefghijklmnopqrstu1!"),
        ("password-sample-no-uppercase", "abc1!abc"),
        ("password-sample-no-lowercase", "ABC1!ABC"),
        ("password-sample-no-number", "Abc!Abcd"),
        ("password-sample-no-symbol", "Abc1Abcd"),
        ("password-sample-valid", "Abc1!abc"),
    ]


class DebugMenu:
    """Debug menu screen"""
    
    ITEMS = [
        ("LockSmith", "locksmith"),
    ]
    
    @staticmethod
    def show():
        """Display the debug menu and return selected item"""
        print("\n" + "=" * 50)
        print("  Debug Menu")
        print("=" * 50)
        for idx, (name, _) in enumerate(DebugMenu.ITEMS, 1):
            print(f"  {idx}. {name}")
        print("  0. Exit")
        print("=" * 50)
        
        while True:
            try:
                choice = input("\nSelect an option: ").strip()
                if choice == "0":
                    return None
                idx = int(choice) - 1
                if 0 <= idx < len(DebugMenu.ITEMS):
                    return DebugMenu.ITEMS[idx][1]
                else:
                    print("Invalid option. Please try again.")
            except ValueError:
                print("Invalid input. Please enter a number.")
            except KeyboardInterrupt:
                print("\n\nExiting...")
                return None


class PasswordValidationScreen:
    """Password validation screen with locale selection"""
    
    def __init__(self, locale: str = "en"):
        self.locale = locale
        self.validator = locksmith.PasswordValidator()
        locksmith.initialize_localization()
        locksmith.set_app_locale(locale)
    
    def clear_screen(self):
        """Clear the terminal screen"""
        import os
        os.system('clear' if os.name != 'nt' else 'cls')
    
    def show_title(self):
        """Display the title"""
        title = locksmith.get_rust_demo_title("Python")
        print("\n" + "=" * 50)
        print(f"  {title}")
        print("=" * 50)
    
    def show_locale_selector(self):
        """Show locale selection menu"""
        print("\n" + locksmith.get_localized_text("locale-picker-label") + ":")
        locales = LocaleOption.all()
        for idx, (code, name) in enumerate(locales, 1):
            marker = "✓" if code == self.locale else " "
            print(f"  {marker} {idx}. {name}")
        print("  0. Back to Debug Menu")
    
    def change_locale(self) -> bool:
        """Allow user to change locale. Returns True if should continue, False if back."""
        self.show_locale_selector()
        
        while True:
            try:
                choice = input("\nSelect locale: ").strip()
                if choice == "0":
                    return False
                
                idx = int(choice) - 1
                locales = LocaleOption.all()
                if 0 <= idx < len(locales):
                    code, name = locales[idx]
                    self.locale = code
                    locksmith.set_app_locale(code)
                    return True
                else:
                    print("Invalid option. Please try again.")
            except ValueError:
                print("Invalid input. Please enter a number.")
            except KeyboardInterrupt:
                return False
    
    def show_sample_passwords(self):
        """Display sample password options"""
        print("\n" + locksmith.get_localized_text("password-sample-header"))
        samples = SamplePassword.SAMPLES
        for idx, (key, password) in enumerate(samples, 1):
            label = locksmith.get_localized_text(key)
            print(f"  {idx}. •{label}")
        print("  0. Enter password manually")
    
    def get_password_input(self) -> Optional[str]:
        """Get password from user (either sample or manual input)"""
        self.show_sample_passwords()
        
        while True:
            try:
                choice = input("\nSelect option: ").strip()
                if choice == "0":
                    password = getpass("Enter password (hidden): ")
                    if not password:
                        password = input("Enter password (visible): ").strip()
                    return password if password else None
                
                idx = int(choice) - 1
                samples = SamplePassword.SAMPLES
                if 0 <= idx < len(samples):
                    _, password = samples[idx]
                    return password
                else:
                    print("Invalid option. Please try again.")
            except ValueError:
                print("Invalid input. Please enter a number.")
            except KeyboardInterrupt:
                return None
    
    def validate_password(self, password: str) -> tuple[str, bool]:
        """Validate password and return message and validity"""
        if not password:
            return "", False
        
        message = self.validator.validate_with_message(password)
        valid_text = locksmith.get_localized_text("password-valid")
        is_valid = message == valid_text
        
        return message, is_valid
    
    def show_validation_result(self, password: str, message: str, is_valid: bool):
        """Display validation result"""
        print("\n" + "-" * 50)
        if password:
            # Show password with masking for security
            masked = "*" * len(password) if len(password) > 0 else ""
            print(f"Password: {masked}")
        
        if message:
            color_code = "\033[92m" if is_valid else "\033[91m"  # Green or Red
            reset_code = "\033[0m"
            print(f"Result: {color_code}{message}{reset_code}")
        print("-" * 50)
    
    def show_instructions(self):
        """Show password instructions"""
        instructions = locksmith.get_localized_text("password-instructions")
        print(f"\n{instructions}")
    
    def run(self) -> bool:
        """Run the password validation screen. Returns True to continue, False to go back."""
        while True:
            self.clear_screen()
            self.show_title()
            
            # Show locale selector
            if not self.change_locale():
                return False
            
            self.clear_screen()
            self.show_title()
            print(f"\nLocale: {LocaleOption.from_code(self.locale)[1]}")
            
            # Get password input
            password = self.get_password_input()
            if password is None:
                return False
            
            # Validate password
            message, is_valid = self.validate_password(password)
            
            # Show results
            self.clear_screen()
            self.show_title()
            print(f"\nLocale: {LocaleOption.from_code(self.locale)[1]}")
            self.show_validation_result(password, message, is_valid)
            self.show_instructions()
            
            # Ask what to do next
            print("\nOptions:")
            print("  1. Try another password")
            print("  2. Change locale")
            print("  0. Back to Debug Menu")
            
            while True:
                try:
                    choice = input("\nSelect option: ").strip()
                    if choice == "0":
                        return False
                    elif choice == "1":
                        break  # Continue to get new password
                    elif choice == "2":
                        break  # Continue to locale selection
                    else:
                        print("Invalid option. Please try again.")
                except KeyboardInterrupt:
                    return False


def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(description="LockSmith Demo App")
    parser.add_argument(
        "--locale",
        default="en",
        choices=["en", "es", "fr"],
        help="Initial locale (default: en)"
    )
    args = parser.parse_args()
    
    # Initialize localization
    locksmith.initialize_localization()
    locksmith.set_app_locale(args.locale)
    
    # Main app loop
    while True:
        # Show debug menu
        selection = DebugMenu.show()
        
        if selection is None:
            print("\nGoodbye!")
            break
        
        if selection == "locksmith":
            # Show password validation screen
            screen = PasswordValidationScreen(locale=args.locale)
            if not screen.run():
                # User went back, continue to show menu again
                continue


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\nExiting...")
        sys.exit(0)
















