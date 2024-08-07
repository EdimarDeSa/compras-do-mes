namespace ComprasDoMes.Models.Internacionalizations;

public enum InternacionalizationLanguage
{
    En,
    Pt_BR,
}

public enum InternacionalizationMessage
{
    UserIdExists,
    UserDontExists,
}

public class InternacionalizationMessageDictionary : Dictionary<InternacionalizationMessage, string>
{
    public InternacionalizationMessageDictionary() : base() { }
}

public class InternacionalizationLanguageDictionary : Dictionary<string, InternacionalizationMessageDictionary>
{
    public InternacionalizationLanguageDictionary() : base() { }
}


public class Internacionalization
{
    private readonly InternacionalizationLanguageDictionary _messages;

    public Internacionalization()
    {
        _messages = new InternacionalizationLanguageDictionary
        {
            {
                "pt_br", new InternacionalizationMessageDictionary
                {
                    { InternacionalizationMessage.UserIdExists, "Id de usuário já em uso!" }
                }
            },
            {
                "en", new InternacionalizationMessageDictionary
                {
                    { InternacionalizationMessage.UserIdExists, "User id already exists!" }
                }
            }
        };
    }

    public string GetMessage(string language, InternacionalizationMessage message)
    {
        if ( _messages.TryGetValue(language, out var langDict))
        {
            if (langDict.TryGetValue(message, out var localizedMessage))
            {
                return localizedMessage;
            }
        }
        return string.Empty;
    }
}