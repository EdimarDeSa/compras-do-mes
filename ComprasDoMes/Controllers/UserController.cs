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

    public UserController(ComprasDoMesContext dbConn, Internacionalization internacionalization)
    {
        _dbConn = dbConn;
        _internacionalization = internacionalization;
    }

    [HttpGet]
    public async Task<ActionResult<IEnumerable<UserDTO>>> GetAllUsers()
    {
        return await _dbConn.Users
            .Select(x => UserToDTO(x))
            .ToListAsync();
    }

    [HttpGet("id/{id}")]
    public async Task<ActionResult<UserDTO>> GetUserById(string id)
    {
        var user =  await _dbConn.Users.FindAsync(id);

        if (user == null) return NotFound(_internacionalization.GetMessage(
            "pt_br", InternacionalizationMessage.UserDontExists
        ));

        return Ok(UserToDTO(user));
    }

    [HttpGet("email/{email}")]
    public async Task<ActionResult<UserDTO>> GetUserByEmail(string email)
    {
        var user =  await _dbConn.Users
        .Where( u => u.Email == email )
        .FirstOrDefaultAsync();

        if (user == null) return NotFound();

        return Ok(UserToDTO(user));
    }

    [HttpPost]
    public async Task<ActionResult<UserDTO>> PostUser(UserDTO userDTO)
    {
        EmailExceptions emailExceptions = UserValidations.ValidateEmail(userDTO.Email, "");
        if(!emailExceptions.IsValid()) return BadRequest(emailExceptions.ToString());

        PasswordExceptions passwordExceptions = UserValidations.ValidatePassword(userDTO.Password, "");
        if(!passwordExceptions.IsValid()) return BadRequest(passwordExceptions.ToString());

        Console.WriteLine("", userDTO.Id);

        var existsId = await _dbConn.Users.FindAsync(userDTO.Id);

        if (existsId != null) return BadRequest("User id already exists!");

        var existsEmail = await _dbConn.Users.Where(u => u.Email == userDTO.Email).FirstOrDefaultAsync();

        if (existsEmail != null) return BadRequest("Email in use!");

        User user = new User
        {
            Name = userDTO.Name,
            Email = userDTO.Email,
            Birthdate = userDTO.Birthdate,
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
    public async Task<ActionResult<UserDTO>> UpdateUserEmail(string id, [FromBody] UserEmailDTO updateUserEmailDTO)
    {
        EmailExceptions emailExceptions = UserValidations.ValidateEmail(updateUserEmailDTO.Email, "");
        if( !emailExceptions.IsValid() ) return BadRequest(emailExceptions.ToString());

        var user =  await _dbConn.Users.FindAsync(id);

        if (user == null) return NotFound();

        user = await _dbConn.Users.Where( u => u.Email == updateUserEmailDTO.Email ).FirstAsync();

        if (user == null) return BadRequest("Email already in use!");

        user.Email = updateUserEmailDTO.Email;

        await _dbConn.SaveChangesAsync();

        return NoContent();
    }

    [HttpPatch("update/birthdate/{id}")]
    public async Task<ActionResult<UserDTO>> UpdateUserBirthDate(string id, [FromBody] UserBirthdateDTO updateUserBirthdateDTO)
    {
        var user =  await _dbConn.Users.FindAsync(id);

        if (user == null) return NotFound();

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
