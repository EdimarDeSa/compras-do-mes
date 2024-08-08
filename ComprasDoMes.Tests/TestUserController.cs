using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using ComprasDoMes.Controllers;
using ComprasDoMes.Models;
using ComprasDoMes.Models.UserModel;
using ComprasDoMes.Models.Internacionalizations;

public class UserControllerTests
{
    private readonly ComprasDoMesContext _context;
    private readonly UserController _controller;
    private readonly Internacionalization _internacionalization;
    private readonly UserValidations _userValidations;
    private readonly List<UserDTO> _users;
    private readonly InternacionalizationLanguage _language;


    public UserControllerTests()
    {
        var options = new DbContextOptionsBuilder<ComprasDoMesContext>()
        .UseInMemoryDatabase(Guid.NewGuid().ToString())
        .Options;

        _context = new(options);
        _internacionalization = new();
        _userValidations = new();
        _controller = new(_context, _internacionalization, _userValidations);
        _users =
        [
            new UserDTO
            {
                Id = "1",
                Name = "John Doe",
                Email = "john@example.com",
                Password = "Str1ng@_s213123",
                Birthdate = new DateOnly(1995, 9, 4)
            }
,
            new UserDTO
            {
                Id = "2",
                Name = "Edimar Freitas",
                Email = "Edimar@email.com",
                Password = "Str1ng@s213123",
                Birthdate = new DateOnly(1995, 9, 4)
            }
        ];
        _language = InternacionalizationLanguage.Pt_BR;
    }

    
    [Fact]
    public async Task PostUser_CreatesNewUser()
    {
        // Arrange
        UserDTO userDTO = _users[0];

        // Act
        var result = await _controller.PostUser(userDTO, InternacionalizationLanguage.Pt_BR);

        // Assert
        var actionResult = Assert.IsType<ActionResult<UserDTO>>(result);
        var createdAtActionResult = Assert.IsType<CreatedAtActionResult>(actionResult.Result);
        var returnValue = Assert.IsType<UserDTO>(createdAtActionResult.Value);
        Assert.Equal(userDTO.Email, returnValue.Email);

    }

    [Fact]
    public async Task GetAllUsers_ReturnsAllUsers()
    {
        // Arrange
        foreach (UserDTO userDTO in _users)
        {
            User user = new User {
                Id = userDTO.Id,
                Name = userDTO.Name,
                Email = userDTO.Email,
                Birthdate = userDTO.Birthdate,
            };
            user.SetPassword(userDTO.Password);

            _context.Users.Add(user);

            await _context.SaveChangesAsync();
        }

        // Act
        var result = await _controller.GetAllUsers();

        // Assert
        var actionResult = Assert.IsType<ActionResult<IEnumerable<UserDTO>>>(result);
        var returnValue = Assert.IsType<List<UserDTO>>(actionResult.Value);
        Assert.Equal(2, returnValue.Count);
    }

    [Fact]
    public async Task GetUserById_ReturnsUser_WhenUserExists()
    {
        // Arrange
        var userDTO = _users[0];
        var user = new User
        {
            Name = userDTO.Name,
            Email = userDTO.Email,
            Birthdate = userDTO.Birthdate,
        };
        user.SetPassword(userDTO.Password);

        _context.Users.Add(user);
        await _context.SaveChangesAsync();

        string id = user.Id;

        // Act
        var result = await _controller.GetUserById(id, _language);

        // Assert
        var actionResult = Assert.IsType<ActionResult<UserDTO>>(result);
        var returnValue = Assert.IsType<OkObjectResult>(actionResult.Result);
        var _userDTO = Assert.IsType<UserDTO>(returnValue.Value);
        Assert.Equal(id, _userDTO.Id);
    }

    [Fact]
    public async Task GetUserById_ReturnsNotFound_WhenUserDoesNotExist()
    {
        // Arrange
        string id = "0";

        // Act
        var result = await _controller.GetUserById(id, _language);

        // Assert
        Assert.IsType<NotFoundObjectResult>(result.Result);
    }

    [Fact]
    public async Task GetUserByEmail_ReturnsUser_WhenUserExists()
    {
        // Arrange
        string email = "john@example.com";

        var userDTO = _users[0];
        var user = new User
        {
            Id = userDTO.Id,
            Name = userDTO.Name,
            Email = userDTO.Email,
            Birthdate = userDTO.Birthdate,
        };
        user.SetPassword(userDTO.Password);

        _context.Users.Add(user);
        await _context.SaveChangesAsync();


        // Act
        var result = await _controller.GetUserByEmail(email, _language);

        // Assert
        var actionResult = Assert.IsType<ActionResult<UserDTO>>(result);
        var returnValue = Assert.IsType<OkObjectResult>(actionResult.Result);
        var _userDTO = Assert.IsType<UserDTO>(returnValue.Value);
        Assert.Equal(userDTO.Id, _userDTO.Id);
    }

    [Fact]
    public async Task GetUserByEmail_ReturnsNotFound_WhenUserDoesNotExist()
    {
        // Arrange
        string email = "qualqueremailquenaoexiste@issomesmo.enois";

        // Act
        var result = await _controller.GetUserByEmail(email, _language);

        // Assert
        Assert.IsType<NotFoundObjectResult>(result.Result);
    }

    [Fact]
    public async Task UpdateUserEmail_ReturnsNoContent_WhenUserExists()
    {
        // Given
        UserDTO userDTO = _users[0];
        var user = new User
        {
            Name = userDTO.Name,
            Email = userDTO.Email,
            Birthdate = userDTO.Birthdate,
        };
        user.SetPassword(userDTO.Password);

        _context.Users.Add(user);
        await _context.SaveChangesAsync();

        string newEmail = "novoemail@email.com";
        
        UserEmailDTO userEmailDTO = new UserEmailDTO
        {
            Email = newEmail
        };

        // When
        var result = await _controller.UpdateUserEmail(user.Id, userEmailDTO, _language);
    
        // Then
        Assert.IsType<NoContentResult>(result.Result);
    }

    [Fact]
    public async Task UpdateUserEmail_ReturnsNotFound_WhenUserDoesNotExists()
    {
        // Given
        UserDTO userDTO = _users[0];

        string newEmail = "novoemail@email.com";
        
        UserEmailDTO userEmailDTO = new UserEmailDTO{
            Email = newEmail
        };

        // When
        var result = await _controller.UpdateUserEmail(userDTO.Id, userEmailDTO, _language);

        // Then
        Assert.IsType<NotFoundObjectResult>(result.Result);
    }

    [Fact]
    public async Task UpdateUserBirthdate_ReturnsNoContent_WhenUserExists()
    {
        // Given
        UserDTO userDTO = _users[0];
        var user = new User
        {
            Id = userDTO.Id,
            Name = userDTO.Name,
            Email = userDTO.Email,
            Birthdate = userDTO.Birthdate,
        };
        user.SetPassword(userDTO.Password);

        _context.Users.Add(user);
        await _context.SaveChangesAsync();

        DateOnly newBirthdate = new DateOnly(1995, 12, 22);
        
        UserBirthdateDTO userBirthdateDTO = new UserBirthdateDTO {
            Birthdate = newBirthdate,
        };

        // When
        var result = await _controller.UpdateUserBirthDate(user.Id, userBirthdateDTO, _language);
    
        // Then
        Assert.IsType<NoContentResult>(result.Result);
    }

    [Fact]
    public async Task UpdateUserBirthdate_ReturnsNoContent_WhenUserDoesNotExists()
    {
        // Given
        UserDTO userDTO = _users[0];

        DateOnly newBirthdate = new DateOnly(1995, 12, 22);

        UserBirthdateDTO userBirthdateDTO = new UserBirthdateDTO {
            Birthdate = newBirthdate,
        };

        // When
        var result = await _controller.UpdateUserBirthDate(userDTO.Id, userBirthdateDTO, _language);

        // Then
        Assert.IsType<NotFoundObjectResult>(result.Result);
    }
}
