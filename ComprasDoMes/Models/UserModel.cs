using System.Text.RegularExpressions;
using ComprasDoMes.Exceptions;
using ComprasDoMes.Exceptions.IValidationExceptions;
using ComprasDoMes.Models.CommomDataModel;

namespace ComprasDoMes.Models.UserModel;
public class User : CommomData
{
    public required string FirstName { get; set; }
    public required string LastName { get; set; }
    private string Password { get; set; } = "";
    public required string Email { get; set; }
    public DateOnly Birthdate { get; set; }

    public bool SetPassword(string newPassword)
    {
        PasswordExceptions errors = UserValidations.ValidatePassword(newPassword, Password);

        if (!errors.IsValid())
        {
            ShowErrors(errors);
            return false;
        }

        Password = newPassword;
        return true;
    }
    
    public bool SetEmail(string newEmail)
    {
        EmailExceptions errors = UserValidations.ValidateEmail(newEmail, Email);

        if (!errors.IsValid())
        {
            ShowErrors(errors);
            return false;
        }

        Email = newEmail;
        return true;
    }

    public bool SetBirthDate(DateOnly newBirthDate)
    {
        if (newBirthDate == Birthdate) return false;
        Birthdate = newBirthDate;
        return true;
    }

    private static void ShowErrors(IValidationExceptions errors)
    {
        foreach (var error in errors.GetAllExceptions())
        {
            Console.WriteLine($"Erro: {error.Key} - {error.Value.ToString()}");
        }
    }
}
public class UserDTO : CommomDataDTO
{
    public required string FirstName { get; set; }
    public required string LastName { get; set; }
    public required string Email { get; set; }
    public required string Password { get; set; }
    public DateOnly Birthdate { get; set; }
    public string FullName => $"{FirstName} {LastName}";

    public override string ToString()
    {
        return $@"Id: {Id}
Name: {Name}
FirstName: {FirstName}
LastName: {LastName}
FullName: {FullName}
Email: {Email}
Birthdate: {Birthdate}";
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


public static class UserValidations
{
        public static PasswordExceptions ValidatePassword(string newPassword, string current)
    {
        PasswordExceptions _errors = new PasswordExceptions();

        if (newPassword == current)
        {
            _errors.Add("Senha identica", new PasswordException("A nova senha deve ser diferente da anterior."));
        }

        if (newPassword.Length < 8)
        {
            _errors.Add("Tamanho insuficiente", new PasswordException("A senha deve conter pelo menos 8 dígitos"));
        }

        if (!Regex.IsMatch(newPassword, @"[A-Z]"))
        {
            _errors.Add("Sem letras maiúsculas", new PasswordException("A senha deve conter ao menos 1 letra maiúscula."));
        }

        if (!Regex.IsMatch(newPassword, @"[a-z]"))
        {
            _errors.Add("Sem letras minúsculas", new PasswordException("A senha deve conter ao menos 1 letra minuscula."));
        }

        if (!Regex.IsMatch(newPassword, @"[0-9]"))
        {
            _errors.Add("Sem letras minúsculas", new PasswordException("A senha deve conter ao menos 1 digito numérico."));
        }

        if (!Regex.IsMatch(newPassword, @"[\W_]"))
        {
            _errors.Add("Sem caracter especial", new PasswordException("A senha deve conter caracteres especiais."));
        }

        return _errors;
    }

    public static EmailExceptions ValidateEmail(string newEmail, string current)
    {
        EmailExceptions _errors = new EmailExceptions();

        if (newEmail == current)
        {
            _errors.Add("E-mail similar", new EmailException("O e-mail deve ser differente do anterior."));
        }

        if (!Regex.IsMatch(newEmail, @".+\@\w+\.\w+"))
        {
            _errors.Add("E-mail inválido", new EmailException("O e-mail deve ter a seguinte estrutura:\nexemplo@dominio.com"));
        }

        return _errors;
    }
}