using System.Text.RegularExpressions;
using ComprasDoMes.Exceptions;
using ComprasDoMes.Exceptions.IValidationExceptions;
using ComprasDoMes.Models.CommomDataModel;
using ComprasDoMes.Models.Internacionalizations;

namespace ComprasDoMes.Models.UserModel;
public class User : CommomUserData
{
    private string Password { get; set; } = "";
    public required string Email { get; set; }
    public DateOnly Birthdate { get; set; }
    private readonly UserValidations _userValidations;

    public User()
    {
        _userValidations = new();
    } 

    public bool SetPassword(string newPassword)
    {
        PasswordExceptions errors = _userValidations.ValidatePassword(newPassword, Password, Language);

        if (!errors.IsValid()) return false;

        Password = newPassword;
        return true;
    }
    
    public bool SetEmail(string newEmail)
    {
        EmailExceptions errors = _userValidations.ValidateEmail(newEmail, Email, Language);

        if (!errors.IsValid()) return false;

        Email = newEmail;
        return true;
    }

    public bool SetBirthDate(DateOnly newBirthDate)
    {
        if (newBirthDate == Birthdate) return false;
        if (Birthdate.Year < 18) return false;

        Birthdate = newBirthDate;

        return true;
    }
}

public class UserDTO : CommomUserData
{
    public required string Email { get; set; }
    public required string Password { get; set; }
    public DateOnly Birthdate { get; set; }

    public override string ToString()
    {
        return $@"Id: {Id}
Name: {Name}
Email: {Email}
Birthdate: {Birthdate}
Creation Date: {Creation}";
    }
}

public class UserEmailDTO
{
    public required string Email { get; set; }

    public override string ToString()
    {
        return $"Email: {Email}";
    }
}

public class UserBirthdateDTO
{
    public required DateOnly Birthdate { get; set; }

    public override string ToString()
    {
        return $"Birthdate: {Birthdate}";
    }
}

public class UserValidations
{
    public readonly Internacionalization _internacionalization;

    public UserValidations()
    {
        _internacionalization = new();
    }
    

    public PasswordExceptions ValidatePassword(string newPassword, string currentPassword, InternacionalizationLanguage language)
    {
        PasswordExceptions _errors = new PasswordExceptions();
        List<InternacionalizationMessage> errorList = new();

        if (newPassword == currentPassword) 
            errorList.Add(InternacionalizationMessage.PasswordSame);

        if (newPassword.Length < 8) 
            errorList.Add(InternacionalizationMessage.PasswordLenght);

        if (!Regex.IsMatch(newPassword, @"[A-Z]"))
            errorList.Add(InternacionalizationMessage.PasswordNeedUpperCase);

        if (!Regex.IsMatch(newPassword, @"[a-z]"))
            errorList.Add(InternacionalizationMessage.PasswordNeedLowerCase);

        if (!Regex.IsMatch(newPassword, @"[0-9]"))
            errorList.Add(InternacionalizationMessage.PasswordNeedNumber);

        if (!Regex.IsMatch(newPassword, @"[\W_]"))
            errorList.Add(InternacionalizationMessage.PasswordNeedSpecial);

        foreach (InternacionalizationMessage message in errorList)
        {
            _errors.Add(message, new PasswordException(_internacionalization.GetMessage(language, message)));
        }
        return _errors;
    }

    public EmailExceptions ValidateEmail(string newEmail, string currentEmail, InternacionalizationLanguage language)
    {
        EmailExceptions _errors = new EmailExceptions();
        List<InternacionalizationMessage> errorList = new();

        if (newEmail == currentEmail)
            errorList.Add(InternacionalizationMessage.EmailSame);

        if (!Regex.IsMatch(newEmail, @".+\@\w+\.\w+"))
            errorList.Add(InternacionalizationMessage.EmailInvalid);

        foreach (InternacionalizationMessage message in errorList)
        {
            _errors.Add(message, new EmailException(_internacionalization.GetMessage(language, message)));
        }

        return _errors;
    }
}