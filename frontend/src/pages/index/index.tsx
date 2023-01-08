import React from 'react';
import { NavLink } from 'react-router-dom';
import { ctx } from '../..';
import Cookies from 'universal-cookie';


function Index() {
  const cookies = new Cookies();

  const {isLoggedin, setIsLoggedin} = React.useContext(ctx);
  return (
    <div className="App">
      Home 

      <NavLink to="/migrations">Migrations</NavLink><br />
      <NavLink to="/login">Login</NavLink>
      <button onClick={() => {
        cookies.remove('token');
        setIsLoggedin(false);
      }}>Logout</button>
    </div>
  );
}

export default Index;
