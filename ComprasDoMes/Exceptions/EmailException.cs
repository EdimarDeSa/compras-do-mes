namespace ComprasDoMes.Exceptions
{
    using ComprasDoMes.Exceptions.ValidationExceptionsBases;

    public class EmailException : BaseValidationException
    {
        public EmailException(string Message) : base(Message)
        {  }
    }

    public class EmailExceptions : BaseValidationExceptions
    {  }
}