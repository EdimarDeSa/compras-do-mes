using ComprasDoMes.Models.Internacionalizations;

namespace ComprasDoMes.Exceptions.IValidationExceptions
{  
    public interface IValidationExceptions
    {
        IDictionary<InternacionalizationMessage, Exception> Errors { get; }
        void Add(InternacionalizationMessage error, Exception errorInstance);
        void Remove(InternacionalizationMessage error);
        IReadOnlyDictionary<InternacionalizationMessage, Exception> GetAllExceptions();
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
        protected readonly Dictionary<InternacionalizationMessage, Exception> _errors = new();
        public IDictionary<InternacionalizationMessage, Exception> Errors => _errors;

        public void Add(InternacionalizationMessage error, Exception errorInstance)
        {
            if(_errors.ContainsKey(error)) return;

            _errors.Add(error, errorInstance);
        }

        public void Remove(InternacionalizationMessage error)
        {
            if(!_errors.ContainsKey(error)) return;

            _errors.Remove(error);
        }

        public IReadOnlyDictionary<InternacionalizationMessage, Exception> GetAllExceptions()
        {
            return new ReadOnlyDictionary<InternacionalizationMessage, Exception>(Errors);
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