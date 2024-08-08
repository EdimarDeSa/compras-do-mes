using System.Runtime.Serialization;

namespace ComprasDoMes.Models.Internacionalizations;

// [JsonConverter(typeof(StringEnumConverter))]
public enum InternacionalizationLanguage
{
    [EnumMember(Value = "Pt_br")]
    En,
    Pt_BR,
}

public enum InternacionalizationMessage
{
    UserIdExists,
    UserNotFound,
    UserEmailUsed,
    PasswordSame,
    PasswordLenght,
    PasswordNeedUpperCase,
    PasswordNeedLowerCase,
    PasswordNeedSpecial,
    PasswordNeedNumber,
    EmailSame,
    EmailInvalidFormat,
    EmailInvalid,
}

public class InternacionalizationMessageDictionary : Dictionary<InternacionalizationMessage, string>
{
    public InternacionalizationMessageDictionary() : base() { }
}

public class InternacionalizationLanguageDictionary : Dictionary<InternacionalizationLanguage, InternacionalizationMessageDictionary>
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
                InternacionalizationLanguage.Pt_BR, new InternacionalizationMessageDictionary
                {
                    { InternacionalizationMessage.UserIdExists, "Id de usuário já em uso!" },
                    { InternacionalizationMessage.UserNotFound, "Usuário não encontrado!" },
                    { InternacionalizationMessage.UserEmailUsed, "E-mail já em uso!" },
                    { InternacionalizationMessage.PasswordSame, "A nova senha deve ser diferente da anterior." },
                    { InternacionalizationMessage.PasswordLenght, "A senha deve conter mais de 8 dígitos." },
                    { InternacionalizationMessage.PasswordNeedUpperCase, "A senha deve conter ao menos 1 letra maiúscula." },
                    { InternacionalizationMessage.PasswordNeedLowerCase, "A senha deve conter ao menos 1 letra minuscula." },
                    { InternacionalizationMessage.PasswordNeedNumber, "A senha deve conter ao menos 1 digito numérico." },
                    { InternacionalizationMessage.PasswordNeedSpecial, "A senha deve conter caracteres especiais." },
                    { InternacionalizationMessage.EmailSame, "O e-mail deve ser differente do anterior." },
                    { InternacionalizationMessage.EmailInvalid, "O e-mail deve ter a seguinte estrutura: exemplo@dominio.com" }
                }
            },
            {
                InternacionalizationLanguage.En, new InternacionalizationMessageDictionary
                {            
                    { InternacionalizationMessage.UserIdExists, "User id already exists!" },
                    { InternacionalizationMessage.UserNotFound, "User does not exist!" },
                    { InternacionalizationMessage.PasswordSame, "The new password cannot be the same as the current one." },
                    { InternacionalizationMessage.PasswordLenght, "The password must be at least 8 characters long." },
                    { InternacionalizationMessage.PasswordNeedUpperCase, "The password must contain at least one uppercase letter." },
                    { InternacionalizationMessage.PasswordNeedLowerCase, "The password must contain at least one lowercase letter." },
                    { InternacionalizationMessage.PasswordNeedNumber, "The password must contain at least one number." },
                    { InternacionalizationMessage.PasswordNeedSpecial, "The password must contain at least one special character." },
                    { InternacionalizationMessage.EmailSame, "The email must be different from the current one." },
                    { InternacionalizationMessage.EmailInvalid, "The email must be in the format: example@domain.com" }
                }
            }
        };
    }

    public string GetMessage(InternacionalizationLanguage language, InternacionalizationMessage message)
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
