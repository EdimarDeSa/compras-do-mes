namespace ComprasDoMes.Exceptions
{
    using ComprasDoMes.Exceptions.ValidationExceptionsBases;

    public class PasswordException : BaseValidationException
    {
        public PasswordException(string message) : base(message)
        {  }
    }

    public class PasswordExceptions : BaseValidationExceptions
    {  }
}