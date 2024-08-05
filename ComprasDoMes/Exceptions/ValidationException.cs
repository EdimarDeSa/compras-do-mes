using Microsoft.AspNetCore.Diagnostics;

namespace ComprasDoMes.Exceptions.IValidationExceptions
{  
    public interface IValidationExceptions
    {
        IDictionary<string, Exception> Errors { get; }
        void Add(string error, Exception errorInstance);
        void Remove(string error);
        IReadOnlyDictionary<string, Exception> GetAllExceptions();
        bool IsValid();
    }
}

namespace ComprasDoMes.Exceptions.ValidationExceptionsBases
{
    using System.Collections.ObjectModel;
    using ComprasDoMes.Exceptions.IValidationExceptions;

    public abstract class BaseValidationException : Exception
    {
        public BaseValidationException(string message) : base(message)
        {  }        
    }

    public abstract class BaseValidationExceptions : IValidationExceptions
    {
        protected readonly Dictionary<string, Exception> _errors = new();
        public IDictionary<string, Exception> Errors => _errors;

        public void Add(string error, Exception errorInstance)
        {
            if(_errors.ContainsKey(error)) return;

            _errors.Add(error, errorInstance);
        }

        public void Remove(string error)
        {
            if(!_errors.ContainsKey(error)) return;

            _errors.Remove(error);
        }

        public IReadOnlyDictionary<string, Exception> GetAllExceptions()
        {
            return new ReadOnlyDictionary<string, Exception>(Errors);
        }

        public bool IsValid()
        {
            return _errors.Count == 0;
        }

        public override string ToString()
        {
            return string.Join("\n", _errors.Select(e => $"Erro: {e.Key} - {e.Value.Message}"));
        }
    }
}