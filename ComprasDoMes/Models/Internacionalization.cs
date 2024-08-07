namespace ComprasDoMes.Models.Internacionalizations;

public enum InternacionalizationLang
{
    En,
    Pt_BR
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

public class InternacionalizationLanguageDictionary : Dictionary<InternacionalizationLang, InternacionalizationMessageDictionary>
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
                InternacionalizationLang.Pt_BR, new InternacionalizationMessageDictionary
                {
                    { InternacionalizationMessage.UserIdExists, "Id de usuário já em uso!" },
                    { InternacionalizationMessage.UserDontExists, "Usuário não existe!"}
                }
            },
            {
                InternacionalizationLang.En, new InternacionalizationMessageDictionary
                {
                    { InternacionalizationMessage.UserIdExists, "User id already exists!" }
                }
            }
        };
    }

    public string GetMessage(InternacionalizationLang language, InternacionalizationMessage message)
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