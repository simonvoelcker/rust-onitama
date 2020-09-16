import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Card extends Component {

  getClasses () {
    let classes = 'card'
    if (this.context.state.selection.card === this.props.name) {
      classes += ' selected-card'
    }
    if (this.props.mini) {
      classes += ' mini-card'
    }
    return classes
  }

  getImageSrc () {
    return 'http://localhost:3030/static/card-' + this.props.name + '.jpg'
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <img onClick={() => mutations.onCardClick(this.props.name)}
               className={this.getClasses()}
               src={this.getImageSrc()}
               alt='card' />
        )}
      </AppConsumer>
    )
  }
}

Card.contextType = AppContext
