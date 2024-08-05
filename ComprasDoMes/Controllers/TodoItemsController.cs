// using Microsoft.AspNetCore.Mvc;
// using Microsoft.EntityFrameworkCore;
// using compras_do_mes.Models;

// namespace TodoApi.Controllers;

// [Route("api/[controller]")]
// [ApiController]
// public class TodoItemsController : ControllerBase
// {
//     private readonly ComprasDoMesContext _dbConn;

//     public TodoItemsController(ComprasDoMesContext dbConn)
//     {
//         _dbConn = dbConn;
//     }

//     // GET: api/TodoItems
//     [HttpGet]
//     public async Task<ActionResult<IEnumerable<TodoItemDTO>>> GetTodoItems()
//     {
//         return await _dbConn.TodoItems
//             .Select(x => ItemToDTO(x))
//             .ToListAsync();
//     }

//     // GET: api/TodoItems/5
//     // <snippet_GetByID>
//     [HttpGet("{id}")]
//     public async Task<ActionResult<TodoItemDTO>> GetTodoItem(long id)
//     {
//         var todoItem = await _dbConn.TodoItems.FindAsync(id);

//         if (todoItem == null)
//         {
//             return NotFound();
//         }

//         return ItemToDTO(todoItem);
//     }
//     // </snippet_GetByID>

//     // PUT: api/TodoItems/5
//     // To protect from overposting attacks, see https://go.microsoft.com/fwlink/?linkid=2123754
//     // <snippet_Update>
//     [HttpPut("{id}")]
//     public async Task<IActionResult> PutTodoItem(long id, TodoItemDTO todoDTO)
//     {
//         if (id != todoDTO.Id)
//         {
//             return BadRequest();
//         }

//         var todoItem = await _dbConn.TodoItems.FindAsync(id);
//         if (todoItem == null)
//         {
//             return NotFound();
//         }

//         todoItem.Title = todoDTO.Title;
//         todoItem.IsComplete = todoDTO.IsComplete;

//         try
//         {
//             await _dbConn.SaveChangesAsync();
//         }
//         catch (DbUpdateConcurrencyException) when (!TodoItemExists(id))
//         {
//             return NotFound();
//         }

//         return NoContent();
//     }
//     // </snippet_Update>

//     // POST: api/TodoItems
//     // To protect from overposting attacks, see https://go.microsoft.com/fwlink/?linkid=2123754
//     // <snippet_Create>
//     [HttpPost]
//     public async Task<ActionResult<TodoItemDTO>> PostTodoItem(TodoItemDTO todoDTO)
//     {
//         var todoItem = new TodoItem
//         {
//             IsComplete = todoDTO.IsComplete,
//             Title = todoDTO.Title
//         };

//         _dbConn.TodoItems.Add(todoItem);
//         await _dbConn.SaveChangesAsync();

//         return CreatedAtAction(
//             nameof(GetTodoItem),
//             new { id = todoItem.Id },
//             ItemToDTO(todoItem)
//         );
//     }
//     // </snippet_Create>

//     // DELETE: api/TodoItems/5
//     [HttpDelete("{id}")]
//     public async Task<IActionResult> DeleteTodoItem(long id)
//     {
//         var todoItem = await _dbConn.TodoItems.FindAsync(id);
//         if (todoItem == null)
//         {
//             return NotFound();
//         }

//         _dbConn.TodoItems.Remove(todoItem);
//         await _dbConn.SaveChangesAsync();

//         return NoContent();
//     }

//     private bool TodoItemExists(long id)
//     {
//         return _dbConn.TodoItems.Any(e => e.Id == id);
//     }

//     private static TodoItemDTO ItemToDTO(TodoItem todoItem) =>
//        new TodoItemDTO
//        {
//            Id = todoItem.Id,
//            Title = todoItem.Title,
//            IsComplete = todoItem.IsComplete
//        };
// }