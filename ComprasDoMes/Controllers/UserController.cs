using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

using ComprasDoMes.Models;
using ComprasDoMes.Models.UserModel;
using ComprasDoMes.Models.Internacionalizations;
using ComprasDoMes.Exceptions;

namespace ComprasDoMes.Controllers;

[Route("api/[Controller]")]
[ApiController]
public class UserController : ControllerBase
{
    private readonly ComprasDoMesContext _dbConn;
    private readonly Internacionalization _internacionalization;
    private readonly UserValidations _userValidations;

    public UserController(ComprasDoMesContext dbConn, Internacionalization internacionalization, UserValidations userValidations)
    {
        _dbConn = dbConn;
        _internacionalization = internacionalization;
        _userValidations = userValidations;
    }

    [HttpGet]
    public async Task<ActionResult<IEnumerable<UserDTO>>> GetAllUsers()
    {
        return await _dbConn.Users
            .Select(x => UserToDTO(x))
            .ToListAsync();
    }

    [HttpGet("id/{id}")]
    public async Task<ActionResult<UserDTO>> GetUserById(string id, [FromQuery] InternacionalizationLanguage language)
    {
        User? user =  await _dbConn.Users.FindAsync(id);

        if (user == null) return NotFound(_internacionalization.GetMessage(
            language, InternacionalizationMessage.UserNotFound
        ));

        return Ok(UserToDTO(user));
    }

    [HttpGet("email/{email}")]
    public async Task<ActionResult<UserDTO>> GetUserByEmail(string email, [FromQuery] InternacionalizationLanguage language)
    {
        var user =  await _dbConn.Users
        .Where( u => u.Email == email )
        .FirstOrDefaultAsync();

        if (user == null) return NotFound(
            _internacionalization.GetMessage(language, InternacionalizationMessage.UserNotFound)
        );

        return Ok(UserToDTO(user));
    }

    [HttpPost]
    public async Task<ActionResult<UserDTO>> PostUser(UserDTO userDTO, [FromQuery] InternacionalizationLanguage language)
    {
        EmailExceptions emailExceptions = _userValidations.ValidateEmail(userDTO.Email, "", language);
        if(!emailExceptions.IsValid()) return BadRequest(emailExceptions.ToString());

        PasswordExceptions passwordExceptions = _userValidations.ValidatePassword(userDTO.Password, "", language);
        if(!passwordExceptions.IsValid()) return BadRequest(passwordExceptions.ToString());

        var existsId = await _dbConn.Users.FindAsync(userDTO.Id);

        if (existsId != null) return BadRequest(_internacionalization.GetMessage(
            language, InternacionalizationMessage.UserIdExists
        ));

        var existsEmail = await _dbConn.Users.Where(u => u.Email == userDTO.Email).FirstOrDefaultAsync();

        if (existsEmail != null) return BadRequest(
            _internacionalization.GetMessage(language, InternacionalizationMessage.UserEmailUsed)
        );

        User user = new User
        {
            Name = userDTO.Name,
            Email = userDTO.Email,
            Birthdate = userDTO.Birthdate,
            Language = userDTO.Language,
        };
        user.SetPassword(userDTO.Password);

        _dbConn.Users.Add(user);
        await _dbConn.SaveChangesAsync();

        return CreatedAtAction(
            nameof(GetUserById),
            new { id = user.Id },
            UserToDTO(user)
        );
    }

    [HttpPatch("update/email/{id}")]
    public async Task<ActionResult<UserDTO>> UpdateUserEmail(string id, [FromBody] UserEmailDTO updateUserEmailDTO, [FromQuery] InternacionalizationLanguage language)
    {
        User? otherUser = await _dbConn.Users.FirstOrDefaultAsync( usr => usr.Email == updateUserEmailDTO.Email );
 
        if (otherUser != null) return BadRequest(
            _internacionalization.GetMessage(language, InternacionalizationMessage.UserEmailUsed)
        );

        User? user =  await _dbConn.Users.FindAsync(id);

        if (user == null) return NotFound(
            _internacionalization.GetMessage(language, InternacionalizationMessage.UserNotFound)
        );

        EmailExceptions emailExceptions = _userValidations.ValidateEmail(updateUserEmailDTO.Email, user.Email, language);
        if( !emailExceptions.IsValid() ) return BadRequest(emailExceptions.ToString());

        user.Email = updateUserEmailDTO.Email;

        await _dbConn.SaveChangesAsync();

        return NoContent();
    }

    [HttpPatch("update/birthdate/{id}")]
    public async Task<ActionResult<UserDTO>> UpdateUserBirthDate(string id, [FromBody] UserBirthdateDTO updateUserBirthdateDTO, [FromQuery] InternacionalizationLanguage language)
    {
        var user =  await _dbConn.Users.FindAsync(id);

        if (user == null) return NotFound(
            _internacionalization.GetMessage(language, InternacionalizationMessage.UserNotFound)
        );

        user.Birthdate = updateUserBirthdateDTO.Birthdate;

        await _dbConn.SaveChangesAsync();

        return NoContent();
    }

    public static UserDTO UserToDTO(User user) =>
       new UserDTO
       {
           Id = user.Id,
           Name = user.Name,
           Password = "",
           Email = user.Email,
           Birthdate = user.Birthdate,
       };
}
