using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

using ComprasDoMes.Models;
using ComprasDoMes.Models.UserModel;
using ComprasDoMes.Exceptions;

namespace ComprasDoMes.Controllers;

[Route("api/[Controller]")]
[ApiController]
public class UserController : ControllerBase
{
    private readonly ComprasDoMesContext _dbConn;
    public UserController(ComprasDoMesContext dbConn)
    {
        _dbConn = dbConn;
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

        if (user == null) return NotFound();

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

        User user = new User
        {
            Id = userDTO.Id,
            Name = userDTO.Name,
            FirstName = userDTO.FirstName,
            LastName = userDTO.LastName,
            Email = userDTO.Email,
            Birthdate = userDTO.BirthDate,
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
    
    public static UserDTO UserToDTO(User user) =>
       new UserDTO
       {
           Id = user.Id,
           Name = user.Name,
           Password = "",
           FirstName = user.FirstName,
           LastName = user.LastName,
           Email = user.Email,
           BirthDate = user.Birthdate,
       };
}
